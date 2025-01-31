use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59653966: FileFormat = FileFormat {
    id: 59_653_966,
    puid: "wikidata/59653966",
    name: "MySQL Table Definition Format",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
