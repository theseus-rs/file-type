use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901849: FileFormat = FileFormat {
    id: 27_901_849,
    puid: "wikidata/27901849",
    name: "Amateur Data Interchange Format, version 2.1.4",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
