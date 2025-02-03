use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113161974: FileFormat = FileFormat {
    id: 113_161_974,
    source_type: SourceType::Wikidata,
    name: "Act! database file",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
