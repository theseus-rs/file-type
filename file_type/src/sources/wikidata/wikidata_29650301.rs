use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650301: FileFormat = FileFormat {
    id: 29_650_301,
    source_type: SourceType::Wikidata,
    name: "Pack",
    extensions: &["z"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
