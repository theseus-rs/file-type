use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27203789: FileFormat = FileFormat {
    id: 27_203_789,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Spreadsheet, version 1.2",
    extensions: &["ods"],
    media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
    signatures: &[],
    related_formats: &[],
};
