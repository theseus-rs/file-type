use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34303668: FileFormat = FileFormat {
    id: 34_303_668,
    source_type: SourceType::Wikidata,
    name: "Syntactically Awesome StyleSheet",
    extensions: &["sass"],
    media_types: &["text/x-sass"],
    internal_signatures: &[],
    related_formats: &[],
};
