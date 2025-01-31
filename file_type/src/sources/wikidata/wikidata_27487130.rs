use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487130: FileFormat = FileFormat {
    id: 27_487_130,
    puid: "wikidata/27487130",
    name: "Shapefile dBASE table",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
