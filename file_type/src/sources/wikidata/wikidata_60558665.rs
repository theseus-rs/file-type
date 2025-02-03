use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60558665: FileFormat = FileFormat {
    id: 60_558_665,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Spreadsheet, version 2",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
