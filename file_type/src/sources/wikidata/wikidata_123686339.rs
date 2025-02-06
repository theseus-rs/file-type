use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123686339: FileFormat = FileFormat {
    id: 123_686_339,
    source_type: SourceType::Wikidata,
    name: "Digital Negative, version 1.7",
    extensions: &["dng"],
    media_types: &["image/dng"],
    signatures: &[],
    related_formats: &[],
};
