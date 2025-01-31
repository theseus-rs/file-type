use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27203722: FileFormat = FileFormat {
    id: 27_203_722,
    puid: "wikidata/27203722",
    name: "OpenDocument Spreadsheet, version 1.1",
    extensions: &["ods"],
    media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
    internal_signatures: &[],
    related_formats: &[],
};
