use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123686339: FileFormat = FileFormat {
    id: 123_686_339,
    source_type: SourceType::Wikidata,
    name: "Digital Negative, version 1.7",
    extensions: &["dng"],
    media_types: &["image/dng"],
    internal_signatures: &[],
    related_formats: &[],
};
