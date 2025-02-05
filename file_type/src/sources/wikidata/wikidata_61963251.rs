use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61963251: FileFormat = FileFormat {
    id: 61_963_251,
    source_type: SourceType::Wikidata,
    name: "Internet Data Query File",
    extensions: &["idq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
