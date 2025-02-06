use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113557082: FileFormat = FileFormat {
    id: 113_557_082,
    source_type: SourceType::Wikidata,
    name: "Creator Image format",
    extensions: &["cif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
