use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131232260: FileFormat = FileFormat {
    id: 131_232_260,
    source_type: SourceType::Wikidata,
    name: "Allotrope Simple Model",
    extensions: &["json"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
