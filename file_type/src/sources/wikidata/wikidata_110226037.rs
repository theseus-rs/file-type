use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110226037: FileFormat = FileFormat {
    id: 110_226_037,
    puid: "wikidata/110226037",
    name: "Raster Matrix Format",
    extensions: &["rsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
