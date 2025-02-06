use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_10397010: FileFormat = FileFormat {
    id: 10_397_010,
    source_type: SourceType::Wikidata,
    name: ".rmp",
    extensions: &["rmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
