use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27203789: FileFormat = FileFormat {
    id: 27_203_789,
    puid: "wikidata/27203789",
    name: "OpenDocument Spreadsheet, version 1.2",
    extensions: &["ods"],
    media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
    internal_signatures: &[],
    related_formats: &[],
};
