use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27355642: FileFormat = FileFormat {
    id: 27_355_642,
    source_type: SourceType::Wikidata,
    name: "ADRG Source File",
    extensions: &["sou"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
