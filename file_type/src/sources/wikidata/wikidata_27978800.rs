use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27978800: FileFormat = FileFormat {
    id: 27_978_800,
    puid: "wikidata/27978800",
    name: "Spectrum 512 Smooshed",
    extensions: &["sps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
