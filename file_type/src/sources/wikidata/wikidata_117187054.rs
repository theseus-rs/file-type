use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117187054: FileFormat = FileFormat {
    id: 117_187_054,
    source_type: SourceType::Wikidata,
    name: "CD Stomper Design file",
    extensions: &["dsn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
