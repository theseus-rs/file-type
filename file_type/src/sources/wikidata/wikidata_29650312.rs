use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650312: FileFormat = FileFormat {
    id: 29_650_312,
    source_type: SourceType::Wikidata,
    name: "PMA",
    extensions: &["pma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
