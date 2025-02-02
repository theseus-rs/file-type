use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21652057: FileFormat = FileFormat {
    id: 21_652_057,
    source_type: SourceType::Wikidata,
    name: "Game Cache File",
    extensions: &["gcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
