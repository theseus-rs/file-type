use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121913880: FileFormat = FileFormat {
    id: 121_913_880,
    puid: "wikidata/121913880",
    name: "Memory Stick Voice File ADPCM Codec",
    extensions: &["msv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
