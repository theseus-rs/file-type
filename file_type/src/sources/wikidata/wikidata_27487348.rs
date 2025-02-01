use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487348: FileFormat = FileFormat {
    id: 27_487_348,
    puid: "wikidata/27487348",
    name: "Shapefile spatial index part 2",
    extensions: &["sbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
