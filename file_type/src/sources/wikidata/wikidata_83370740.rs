use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83370740: FileFormat = FileFormat {
    id: 83_370_740,
    puid: "wikidata/83370740",
    name: "Audio Visual Research",
    extensions: &["avr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
