use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85415600: FileFormat = FileFormat {
    id: 85_415_600,
    source_type: SourceType::Wikidata,
    name: "Tweet JSON",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
