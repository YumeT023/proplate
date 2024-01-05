#[cfg(test)]
mod dynamic_files {
  use crate::cmd::create::{create, CreateOptions};
  use crate::cmd::tests::{get_sample, new_trash, run_isolated_test};
  use crate::{assert_gen_ok, assert_gen_snapshot, test_create};

  use std::collections::HashMap;
  use std::fs;

  use proplate_core::template::resolver::clone_template;
  use proplate_core::template::META_CONF;

  #[test]
  fn only_pkg() {
    run_isolated_test(
      || {
        let ctx = HashMap::from([
          ("name".to_string(), "only-pkg".to_string()),
          ("ver".to_string(), "1.0.0".to_string()),
          ("file_structure".to_string(), "module".to_string()),
        ]);

        let (path, snap) = test_create!("dynamic_files", "only-pkg", ctx);

        assert_gen_ok!(&path);
        assert_gen_snapshot!(&snap, &path, "main.js" "package.json");

        Ok(())
      },
      /*clean*/ false,
    );
  }

  #[test]
  fn select_both() {
    run_isolated_test(
      || {
        let ctx = HashMap::from([
          ("name".to_string(), "select-both".to_string()),
          ("ver".to_string(), "1.0.0".to_string()),
          ("file_structure".to_string(), "commonjs".to_string()),
        ]);

        let (path, snap) = test_create!("dynamic_files", "select-both", ctx);

        assert_gen_ok!(&path);
        assert_gen_snapshot!(&snap, &path, "main.js" "package.json");

        Ok(())
      },
      /*clean*/ true,
    );
  }
}
