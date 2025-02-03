use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111341669: FileFormat = FileFormat {
    id: 111_341_669,
    source_type: SourceType::Wikidata,
    name: "Creative Labs FM instrument",
    extensions: &["sbi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
