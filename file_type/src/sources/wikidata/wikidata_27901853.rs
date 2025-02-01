use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27901853: FileFormat = FileFormat {
    id: 27_901_853,
    puid: "wikidata/27901853",
    name: "Amateur Data Interchange Format, version 2.1.7",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
