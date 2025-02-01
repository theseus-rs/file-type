use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27902233: FileFormat = FileFormat {
    id: 27_902_233,
    puid: "wikidata/27902233",
    name: "Amateur Data Interchange Format, ADX variant, version 3.0.3",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
