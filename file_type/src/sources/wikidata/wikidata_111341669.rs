use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111341669: FileFormat = FileFormat {
    id: 111_341_669,
    source_type: SourceType::Wikidata,
    name: "Creative Labs FM instrument",
    extensions: &["sbi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
