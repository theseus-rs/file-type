use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27902119: FileFormat = FileFormat {
    id: 27_902_119,
    puid: "wikidata/27902119",
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.2",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
