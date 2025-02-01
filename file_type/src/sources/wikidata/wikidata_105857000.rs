use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857000: FileFormat = FileFormat {
    id: 105_857_000,
    puid: "wikidata/105857000",
    name: "Generic Printer Description - Unidrv minidriver",
    extensions: &["gpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
