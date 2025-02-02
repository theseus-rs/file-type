use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650301: FileFormat = FileFormat {
    id: 29_650_301,
    source_type: SourceType::Wikidata,
    name: "Pack",
    extensions: &["z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
