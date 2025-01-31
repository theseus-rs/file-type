use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27902241: FileFormat = FileFormat {
    id: 27_902_241,
    puid: "wikidata/27902241",
    name: "Amateur Data Interchange Format, ADX variant, version 3.0.4",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
