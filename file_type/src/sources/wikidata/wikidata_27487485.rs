use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487485: FileFormat = FileFormat {
    id: 27_487_485,
    puid: "wikidata/27487485",
    name: "Shapefile spatial index of features part 1",
    extensions: &["fbn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
