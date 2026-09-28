use std::path::Path;

#[derive(Debug)]
pub struct Ancestor<'a> {
    /// The non trivial ancestor(excludes the root "/").
    pub ancestor: Option<&'a Path>,

    pub grand_parent: Option<&'a Path>,
    pub parent: Option<&'a Path>,
}

pub fn path2ancestor<'a>(p: &'a Path) -> Ancestor<'a> {
    let parent: Option<&Path> = p.parent().filter(|p: &&Path| !p.is_empty());
    let grand_parent: Option<&Path> = parent
        .and_then(Path::parent)
        .filter(|p: &&Path| !p.is_empty());

    let ancestor: Option<&Path> = p.ancestors().skip(1).fold(None, |state, next| {
        let ostr = next.as_os_str();
        let encoded: &[u8] = ostr.as_encoded_bytes();
        if encoded.is_empty() {
            return state;
        }

        if encoded == b"/" {
            return state;
        }

        Some(next)
    });

    Ancestor {
        ancestor,
        grand_parent,
        parent,
    }
}

#[cfg(test)]
mod tests {
    mod path2ancestor {
        use crate::Ancestor;
        use std::path::Path;

        #[test]
        fn empty() {
            let a: Ancestor = crate::path2ancestor(Path::new(""));

            assert!(a.ancestor.is_none());
            assert!(a.grand_parent.is_none());
            assert!(a.parent.is_none());
        }

        #[test]
        fn root_only() {
            let a: Ancestor = crate::path2ancestor(Path::new("/"));

            assert!(a.ancestor.is_none());
            assert!(a.grand_parent.is_none());
            assert!(a.parent.is_none());
        }

        #[test]
        fn simple_absolute() {
            let a: Ancestor =
                crate::path2ancestor(Path::new("/home/me/Documents/path/to/mydoc.dat"));

            let ancestor: &Path = a.ancestor.unwrap();
            let grand_parent: &Path = a.grand_parent.unwrap();
            let parent: &Path = a.parent.unwrap();

            assert_eq!(ancestor, Path::new("/home"));
            assert_eq!(grand_parent, Path::new("/home/me/Documents/path"));
            assert_eq!(parent, Path::new("/home/me/Documents/path/to"));
        }

        #[test]
        fn two_levels() {
            let a: Ancestor = crate::path2ancestor(Path::new("/home/user/file.txt"));

            assert_eq!(a.ancestor, Some(Path::new("/home")));
            assert_eq!(a.grand_parent, Some(Path::new("/home")));
            assert_eq!(a.parent, Some(Path::new("/home/user")));
        }

        #[test]
        fn single_level() {
            let a: Ancestor = crate::path2ancestor(Path::new("/file.txt"));

            assert_eq!(a.ancestor, None);
            assert_eq!(a.grand_parent, None);
            assert_eq!(a.parent, Some(Path::new("/")));
        }

        #[test]
        fn relative_path() {
            let a: Ancestor = crate::path2ancestor(Path::new("a/b/c.txt"));

            assert_eq!(a.ancestor, Some(Path::new("a")));
            assert_eq!(a.grand_parent, Some(Path::new("a")));
            assert_eq!(a.parent, Some(Path::new("a/b")));
        }

        #[test]
        fn relative_root_like() {
            let a: Ancestor = crate::path2ancestor(Path::new("a/b/"));

            // ancestors of "a/b/" are: "a/b", "a", ""
            assert_eq!(a.ancestor, Some(Path::new("a")));
            assert_eq!(a.grand_parent, None);
            assert_eq!(a.parent, Some(Path::new("a")));
        }

        #[test]
        fn nested_deep() {
            let a: Ancestor = crate::path2ancestor(Path::new("/a/b/c/d/e/f/g/h/i/j.txt"));

            assert_eq!(a.ancestor, Some(Path::new("/a")));
            assert_eq!(a.grand_parent, Some(Path::new("/a/b/c/d/e/f/g/h")));
            assert_eq!(a.parent, Some(Path::new("/a/b/c/d/e/f/g/h/i")));
        }
    }
}
