use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_30102407: FileFormat = FileFormat {
    id: 30_102_407,
    puid: "wikidata/30102407",
    name: "Amateur Data Interchange Format, ADX variant, version 3.0.5",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
