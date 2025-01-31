use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27902240: FileFormat = FileFormat {
    id: 27_902_240,
    puid: "wikidata/27902240",
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.4",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
