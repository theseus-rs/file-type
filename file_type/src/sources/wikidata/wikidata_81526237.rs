use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_81526237: FileFormat = FileFormat {
    id: 81_526_237,
    source_type: SourceType::Wikidata,
    name: "MapInfo MapBasic tabular DataBase",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
