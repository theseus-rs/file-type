use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117187054: FileFormat = FileFormat {
    id: 117_187_054,
    source_type: SourceType::Wikidata,
    name: "CD Stomper Design file",
    extensions: &["dsn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
