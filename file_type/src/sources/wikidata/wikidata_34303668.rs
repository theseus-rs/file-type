use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34303668: FileFormat = FileFormat {
    id: 34_303_668,
    source_type: SourceType::Wikidata,
    name: "Syntactically Awesome StyleSheet",
    extensions: &["sass"],
    media_types: &["text/x-sass"],
    signatures: &[],
    related_formats: &[],
};
