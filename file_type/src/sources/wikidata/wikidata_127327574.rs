use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127327574: FileFormat = FileFormat {
    id: 127_327_574,
    source_type: SourceType::Wikidata,
    name: "Clojure file",
    extensions: &["clj"],
    media_types: &["application/x-clojure", "text/x-clojure"],
    internal_signatures: &[],
    related_formats: &[],
};
