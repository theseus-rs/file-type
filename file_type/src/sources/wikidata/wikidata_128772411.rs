use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128772411: FileFormat = FileFormat {
    id: 128_772_411,
    source_type: SourceType::Wikidata,
    name: "ClojureScript file format",
    extensions: &["cljs"],
    media_types: &["application/x-clojurescript", "text/x-clojurescript"],
    internal_signatures: &[],
    related_formats: &[],
};
