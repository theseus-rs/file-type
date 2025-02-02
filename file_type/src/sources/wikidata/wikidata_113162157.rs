use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113162157: FileFormat = FileFormat {
    id: 113_162_157,
    source_type: SourceType::Wikidata,
    name: "MyAdvancedLabelDesigner File",
    extensions: &["mlb", "mld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
