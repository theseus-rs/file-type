use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52063298: FileFormat = FileFormat {
    id: 52_063_298,
    puid: "wikidata/52063298",
    name: "Scanstudio 16-Colour Bitmap",
    extensions: &["adc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
