use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113162157: FileFormat = FileFormat {
    id: 113_162_157,
    source_type: SourceType::Wikidata,
    name: "MyAdvancedLabelDesigner File",
    extensions: &["mlb", "mld"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
