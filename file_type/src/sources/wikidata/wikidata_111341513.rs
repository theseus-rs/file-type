use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341513: FileFormat = FileFormat {
    id: 111_341_513,
    puid: "wikidata/111341513",
    name: "Signed byte (8-bit) data",
    extensions: &["sb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
