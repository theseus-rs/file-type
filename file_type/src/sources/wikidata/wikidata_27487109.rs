use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487109: FileFormat = FileFormat {
    id: 27_487_109,
    puid: "wikidata/27487109",
    name: "Shapefile index file",
    extensions: &["shx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
