/// A limited emacs reader that understands just enough to be able to turn the
/// sexps from the org-roam database into objects

use super::testdata::TESTDATA;

enum EmacsObject {
    Nil(()),
    Cons(Box<Cons>),
}

struct Cons {
    car: EmacsObject,
    cdr: EmacsObject
}

