use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116884493: FileFormat = FileFormat {
    id: 116_884_493,
    puid: "wikidata/116884493",
    name: "EPS Tiff Preview",
    extensions: &["eps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
