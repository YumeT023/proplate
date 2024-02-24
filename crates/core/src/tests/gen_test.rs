#[cfg(test)]
mod exclude_files {
  use std::collections::HashMap;

  use crate::{
    assert_gen_ok,
    gen::bootstrap::bootstrap,
    template::{resolver::clone_template, META_CONF},
    test_create,
    tests::{get_sample, new_trash, run_isolated_test},
  };

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
  use std::collections::HashMap;

  use crate::{
    assert_gen_ok, assert_gen_snapshot,
    gen::bootstrap::bootstrap,
    template::{resolver::clone_template, META_CONF},
    test_create,
    tests::{assert_dir_superset, get_sample, new_trash, run_isolated_test},
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
      /*clean*/ false,
    );
  }

  #[test]
  fn recursive_dyn_file() {
    run_isolated_test(
      || {
        let ctx = HashMap::from([
          ("dyn0".to_string(), "val0".to_string()),
          ("d0".to_string(), "v0".to_string()),
          ("d1".to_string(), "v1".to_string()),
          ("d2".to_string(), "v2".to_string()),
        ]);

        let (path, snap) = test_create!("dynamic_files", "recursive-dyn-file", ctx);

        assert_gen_ok!(&path);
        assert_gen_snapshot!(&snap, &path);

        Ok(())
      },
      /*clean*/ false,
    );
  }
}

#[cfg(test)]
mod additional_ops {
  use std::collections::HashMap;

  use crate::{
    assert_gen_ok, assert_gen_snapshot,
    gen::bootstrap::bootstrap,
    template::{resolver::clone_template, META_CONF},
    test_create,
    tests::{assert_dir_superset, get_sample, new_trash, run_isolated_test},
  };

  #[test]
  fn unlicensed() {
    run_isolated_test(
      || {
        let ctx = HashMap::from([
          ("project_name".to_string(), "unlicensed".to_string()),
          ("author_name".to_string(), "Proplate".to_string()),
          ("license".to_string(), "UNLICENSED".to_string()),
        ]);

        let (path, _) = test_create!("additional_ops", "conditional-license", ctx);
        // we got custom snap path here
        let (snap, _) = get_sample("additional_ops", "unlicensed-snapshot");

        assert_gen_ok!(&path);
        assert_gen_snapshot!(&snap, &path);

        Ok(())
      },
      /*clean*/ false,
    );
  }

  #[test]
  fn bsd_2_clause() {
    run_isolated_test(
      || {
        let ctx = HashMap::from([
          ("project_name".to_string(), "bsd-2-clause".to_string()),
          ("author_name".to_string(), "Proplate".to_string()),
          ("license".to_string(), "BSD-2-Clause".to_string()),
        ]);

        let (path, _) = test_create!("additional_ops", "conditional-license", ctx);
        // we got custom snap path here
        let (snap, _) = get_sample("additional_ops", "bsd-2-clause-license-snapshot");

        assert_gen_ok!(&path);
        assert_gen_snapshot!(&snap, &path);

        Ok(())
      },
      /*clean*/ false,
    );
  }

  #[test]
  fn mit() {
    run_isolated_test(
      || {
        let ctx = HashMap::from([
          ("project_name".to_string(), "mit".to_string()),
          ("author_name".to_string(), "Proplate".to_string()),
          ("license".to_string(), "MIT".to_string()),
        ]);

        let (path, _) = test_create!("additional_ops", "conditional-license", ctx);
        // we got custom snap path here
        let (snap, _) = get_sample("additional_ops", "mit-license-snapshot");

        assert_gen_ok!(&path);
        assert_gen_snapshot!(&snap, &path);

        Ok(())
      },
      /*clean*/ true,
    );
  }

  #[test]
  fn copy_dir() {
    run_isolated_test(
      || {
        let ctx = HashMap::new();

        let (path, snap) = test_create!("additional_ops", "copy-dir", ctx);

        assert_gen_ok!(&path);
        assert_gen_snapshot!(&snap, &path);

        Ok(())
      },
      /*clean*/ true,
    );
  }
}
