use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2328734: FileFormat = FileFormat {
    id: 2_328_734,
    source_type: SourceType::Wikidata,
    name: "JISP",
    extensions: &["jisp"],
    media_types: &["application/vnd.jisp"],
    signatures: &[],
    related_formats: &[],
};
