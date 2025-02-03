use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125691821: FileFormat = FileFormat {
    id: 125_691_821,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Master Document",
    extensions: &["odm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
