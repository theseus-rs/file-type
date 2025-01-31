use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901921: FileFormat = FileFormat {
    id: 27_901_921,
    puid: "wikidata/27901921",
    name: "Amateur Data Interchange Format, version 2.2.3",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
