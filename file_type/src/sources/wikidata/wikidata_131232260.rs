use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131232260: FileFormat = FileFormat {
    id: 131_232_260,
    source_type: SourceType::Wikidata,
    name: "Allotrope Simple Model",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
