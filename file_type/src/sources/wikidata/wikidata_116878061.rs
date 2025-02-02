use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116878061: FileFormat = FileFormat {
    id: 116_878_061,
    source_type: SourceType::Wikidata,
    name: "Calendar Creator CSV Event File",
    extensions: &["csv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
