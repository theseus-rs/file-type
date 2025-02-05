use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_62: FileFormat = FileFormat {
    id: 62,
    source_type: SourceType::Linguist,
    name: "Clojure",
    extensions: &[
        "bb", "boot", "cl2", "clj", "cljc", "cljs", "cljs.hl", "cljscm", "cljx", "hic",
    ],
    media_types: &["text/x-clojure"],
    signatures: &[],
    related_formats: &[],
};
