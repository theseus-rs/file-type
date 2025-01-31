use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115055414: FileFormat = FileFormat {
    id: 115_055_414,
    puid: "wikidata/115055414",
    name: "The Spectral Geologist Dataset",
    extensions: &["tsg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
