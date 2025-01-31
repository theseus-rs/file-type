use crate::format::FileFormat;

pub(crate) const LINGUIST_62: FileFormat = FileFormat {
    id: 62,
    puid: "linguist/62",
    name: "Clojure",
    extensions: &[
        "bb", "boot", "cl2", "clj", "cljc", "cljs", "cljs.hl", "cljscm", "cljx", "hic",
    ],
    media_types: &["text/x-clojure"],
    internal_signatures: &[],
    related_formats: &[],
};
