use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116647633: FileFormat = FileFormat {
    id: 116_647_633,
    puid: "wikidata/116647633",
    name: "KeyForm 3.x Document",
    extensions: &["ifd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
