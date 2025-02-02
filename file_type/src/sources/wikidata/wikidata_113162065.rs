use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113162065: FileFormat = FileFormat {
    id: 113_162_065,
    source_type: SourceType::Wikidata,
    name: "Approach database file",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
