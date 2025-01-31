use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100235553: FileFormat = FileFormat {
    id: 100_235_553,
    puid: "wikidata/100235553",
    name: "FARO Laser Scan File",
    extensions: &["fls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
