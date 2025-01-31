use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115055515: FileFormat = FileFormat {
    id: 115_055_515,
    puid: "wikidata/115055515",
    name: "The Spectral Geologist Dataset 7",
    extensions: &["tsg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
