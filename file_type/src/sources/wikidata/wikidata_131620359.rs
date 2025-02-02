use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131620359: FileFormat = FileFormat {
    id: 131_620_359,
    source_type: SourceType::Wikidata,
    name: "Ansys Fluent file format",
    extensions: &["cas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
