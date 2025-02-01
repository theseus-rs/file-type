use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127327574: FileFormat = FileFormat {
    id: 127_327_574,
    puid: "wikidata/127327574",
    name: "Clojure file",
    extensions: &["clj", "clj"],
    media_types: &["application/x-clojure", "text/x-clojure"],
    internal_signatures: &[],
    related_formats: &[],
};
