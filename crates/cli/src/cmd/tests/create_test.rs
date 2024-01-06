#[cfg(test)]
mod exclude_files {
  use crate::cmd::create::{create, CreateOptions};
  use crate::cmd::tests::{get_sample, new_trash, run_isolated_test};
  use crate::{assert_gen_ok, test_create};

  use std::collections::HashMap;

  use proplate_core::template::resolver::clone_template;
  use proplate_core::template::META_CONF;

  #[test]
  fn ban() {
    run_isolated_test(
      || {
        let (path, _) = test_create!("exclude_files", "ban-node-modules", HashMap::new());

        // "meta.json" & ".proplate_aux_utils" is banned by default
        assert_gen_ok!(&path);

        // is cursed node_modules banned!!!
        assert!(!path.join("node_modules").exists());

        Ok(())
      },
      /*clean*/ false,
    );
  }
}

#[cfg(test)]
mod dynamic_files {
  use crate::cmd::create::{create, CreateOptions};
  use crate::cmd::tests::{get_sample, new_trash, run_isolated_test};
  use crate::{assert_gen_ok, assert_gen_snapshot, test_create};

  use std::collections::HashMap;
  use std::fs;

  use proplate_core::{
    fs::walk::walk_dir,
    template::{resolver::clone_template, META_CONF},
  };

  #[test]
  fn empty_dyn_file() {
    run_isolated_test(
      || {
        let ctx = HashMap::from([
          ("name".to_string(), "empty-dyn-file".to_string()),
          ("ver".to_string(), "1.0.0".to_string()),
          ("file_structure".to_string(), "module".to_string()),
        ]);

        let (path, snap) = test_create!("dynamic_files", "empty-dyn-file", ctx);

        assert_gen_ok!(&path);
        assert_gen_snapshot!(&snap, &path);

        Ok(())
      },
      /*clean*/ false,
    );
  }

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
        assert_gen_snapshot!(&snap, &path);

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
        assert_gen_snapshot!(&snap, &path);

        Ok(())
      },
      /*clean*/ true,
    );
  }
}
