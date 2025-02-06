use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131620359: FileFormat = FileFormat {
    id: 131_620_359,
    source_type: SourceType::Wikidata,
    name: "Ansys Fluent file format",
    extensions: &["cas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
