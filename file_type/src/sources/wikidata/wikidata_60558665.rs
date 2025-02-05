use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60558665: FileFormat = FileFormat {
    id: 60_558_665,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Spreadsheet, version 2",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
