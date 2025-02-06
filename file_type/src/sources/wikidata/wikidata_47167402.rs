use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47167402: FileFormat = FileFormat {
    id: 47_167_402,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Spreadsheet file format",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
