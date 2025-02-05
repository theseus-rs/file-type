use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118140187: FileFormat = FileFormat {
    id: 118_140_187,
    source_type: SourceType::Wikidata,
    name: "Serenade Symbol File",
    extensions: &["sym"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
