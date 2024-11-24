use std::{borrow::Cow, marker::PhantomData, mem, path::Path};

use anyhow::Result;
use regex::Regex;
use tokio::fs;

use crate::{Sha256Sum, Version};

use super::super::FileUpdater;

pub struct Updater<'a>(PhantomData<&'a ()>);

pub struct UpdateRequest<'a> {
    pub target_version: &'a Version,
    pub target_sha256sum: &'a Sha256Sum,
}

impl<'a> FileUpdater for Updater<'a> {
    type Request = UpdateRequest<'a>;

    async fn update_file(path: impl AsRef<Path>, request: &Self::Request) -> Result<()> {
        let mut file_contents = fs::read_to_string(path.as_ref()).await?;

        Updater::update_version(&mut file_contents, request.target_version)?;
        Updater::update_sha256hash(&mut file_contents, request.target_sha256sum)?;

        fs::write(path.as_ref(), file_contents).await?;

        Ok(())
    }
}

impl Updater<'_> {
    fn update_arg(
        file_contents: &mut String,
        arg_name: impl AsRef<str>,
        new_value: impl AsRef<str>,
    ) -> Result<()> {
        let re = Regex::new(
            format!(
                r"(?m)^(?<leading>\s*?- {0}\s*=\s*)(.+?)(?<trailing>\s+.+)?$",
                arg_name.as_ref()
            )
            .as_str(),
        )?;
        let target = format!("${{leading}}{0}${{trailing}}", new_value.as_ref());
        let result = re.replace(file_contents, target);

        match result {
            Cow::Borrowed(_) => { /* No replacement happened */ }
            Cow::Owned(mut new) => mem::swap(file_contents, &mut new),
        }

        Ok(())
    }

    fn update_version(file_contents: &mut String, target_version: &Version) -> Result<()> {
        Self::update_arg(file_contents, "VERSION", &**target_version)
    }

    fn update_sha256hash(file_contents: &mut String, target_sha256sum: &Sha256Sum) -> Result<()> {
        Self::update_arg(file_contents, "SHA256", &**target_sha256sum)
    }
}

#[cfg(test)]
mod tests {
    use super::Updater;
    use anyhow::Result;

    #[test]
    fn test_update_version() -> Result<()> {
        for (input, expected) in [
            (("- VERSION=2.0.1", "2.0.2"), "- VERSION=2.0.2"),
            (("  - VERSION=2.0.2", "2.0.3"), "  - VERSION=2.0.3"),
            (
                ("      - VERSION=2.0.3 #comment", "2.0.4"),
                "      - VERSION=2.0.4 #comment",
            ),
            (
                ("    - VERSION=2.0.4 #comment with trailing     ", "2.0.5"),
                "    - VERSION=2.0.5 #comment with trailing     ",
            ),
            (
                (
                    "

                   - VERSION=2.0.5

                ",
                    "2.0.6",
                ),
                "

                   - VERSION=2.0.6

                ",
            ),
            (
                ("\n\n     - VERSION=2.0.6\n\n    - unrelated=5", "2.0.7"),
                "\n\n     - VERSION=2.0.7\n\n    - unrelated=5",
            ),
            (
                (
                    "version: '2'
services:
  space-age-server:
    build:
      context: .
      args:
      # Check buildinfo.json for supported versions and SHAs
      # https://github.com/factoriotools/factorio-docker/blob/master/buildinfo.json
      - VERSION = 2.0.21 #comment, yo!
      - SHA256=1d6d2785006d6a8d9d5fdcdaa7097a189ec35ba95f3521025dc4e046f7a1398e
    restart: always
",
                    "2.0.22",
                ),
                "version: '2'
services:
  space-age-server:
    build:
      context: .
      args:
      # Check buildinfo.json for supported versions and SHAs
      # https://github.com/factoriotools/factorio-docker/blob/master/buildinfo.json
      - VERSION = 2.0.22 #comment, yo!
      - SHA256=1d6d2785006d6a8d9d5fdcdaa7097a189ec35ba95f3521025dc4e046f7a1398e
    restart: always
",
            ),
        ] {
            let mut file_contents = input.0.to_string();
            let target_version = input.1.into();

            Updater::update_version(&mut file_contents, &target_version)?;

            assert_eq!(file_contents, expected);
        }

        Ok(())
    }

    #[test]
    fn test_update_sha256sum() -> Result<()> {
        for (input, expected) in [
            (
                ("- SHA256=1d6d2785000", "abcd2785000"),
                "- SHA256=abcd2785000",
            ),
            (
                ("  - SHA256=1d6d2785001", "abcd2785001"),
                "  - SHA256=abcd2785001",
            ),
            (
                ("  - SHA256=1d6d2785002 #comment", "abcd2785002"),
                "  - SHA256=abcd2785002 #comment",
            ),
            (
                (
                    "version: '2'
services:
  space-age-server:
    build:
      context: .
      args:
      # Check buildinfo.json for supported versions and SHAs
      # https://github.com/factoriotools/factorio-docker/blob/master/buildinfo.json
      - VERSION = 2.0.21 #comment, yo!
      - SHA256=1d6d2785006d6a8d9d5fdcdaa7097a189ec35ba95f3521025dc4e046f7a1398e #blah
    restart: always
",
                    "00000085006d6a8d9d5fdcdaa7097a189ec35ba95f3521025dc4e046f7000000",
                ),
                "version: '2'
services:
  space-age-server:
    build:
      context: .
      args:
      # Check buildinfo.json for supported versions and SHAs
      # https://github.com/factoriotools/factorio-docker/blob/master/buildinfo.json
      - VERSION = 2.0.21 #comment, yo!
      - SHA256=00000085006d6a8d9d5fdcdaa7097a189ec35ba95f3521025dc4e046f7000000 #blah
    restart: always
",
            ),
        ] {
            let mut file_contents = input.0.to_string();
            let target_sha256hash = input.1.into();

            Updater::update_sha256hash(&mut file_contents, &target_sha256hash)?;

            assert_eq!(file_contents, expected);
        }

        Ok(())
    }

    #[test]
    fn test_both() -> Result<()> {
        for (input, expected) in [(
            (
                "version: '2'
services:
  space-age-server:
    build:
      context: .
      args:
      # Check buildinfo.json for supported versions and SHAs
      # https://github.com/factoriotools/factorio-docker/blob/master/buildinfo.json
      - VERSION = 2.0.21 #comment, yo!
      - SHA256=1d6d2785006d6a8d9d5fdcdaa7097a189ec35ba95f3521025dc4e046f7a1398e #blah
    restart: always
",
                "2.0.22",
                "00000085006d6a8d9d5fdcdaa7097a189ec35ba95f3521025dc4e046f7000123",
            ),
            "version: '2'
services:
  space-age-server:
    build:
      context: .
      args:
      # Check buildinfo.json for supported versions and SHAs
      # https://github.com/factoriotools/factorio-docker/blob/master/buildinfo.json
      - VERSION = 2.0.22 #comment, yo!
      - SHA256=00000085006d6a8d9d5fdcdaa7097a189ec35ba95f3521025dc4e046f7000123 #blah
    restart: always
",
        )] {
            let mut file_contents = input.0.to_string();
            let target_version = input.1.into();
            let target_sha256hash = input.2.into();

            Updater::update_version(&mut file_contents, &target_version)?;
            Updater::update_sha256hash(&mut file_contents, &target_sha256hash)?;

            assert_eq!(file_contents, expected);
        }

        Ok(())
    }
}
