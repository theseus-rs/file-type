use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205488: FileFormat = FileFormat {
    id: 28_205_488,
    puid: "wikidata/28205488",
    name: "Windows Cursor",
    extensions: &["cur"],
    media_types: &["image/x-win-bitmap"],
    internal_signatures: &[],
    related_formats: &[],
};
