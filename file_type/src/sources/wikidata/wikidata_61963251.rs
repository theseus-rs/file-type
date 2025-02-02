use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61963251: FileFormat = FileFormat {
    id: 61_963_251,
    source_type: SourceType::Wikidata,
    name: "Internet Data Query File",
    extensions: &["idq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
