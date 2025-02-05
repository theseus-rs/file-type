use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131620450: FileFormat = FileFormat {
    id: 131_620_450,
    source_type: SourceType::Wikidata,
    name: "Ansys Fluent Data file format",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
