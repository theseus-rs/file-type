use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487359: FileFormat = FileFormat {
    id: 27_487_359,
    puid: "wikidata/27487359",
    name: "ArcView GIS 3.x attribute index",
    extensions: &["atx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
