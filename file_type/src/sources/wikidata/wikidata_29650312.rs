use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650312: FileFormat = FileFormat {
    id: 29_650_312,
    source_type: SourceType::Wikidata,
    name: "PMA",
    extensions: &["pma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
