use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60558665: FileFormat = FileFormat {
    id: 60_558_665,
    puid: "wikidata/60558665",
    name: "ClarisWorks Spreadsheet, version 2",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
