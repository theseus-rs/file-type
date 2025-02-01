use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61693036: FileFormat = FileFormat {
    id: 61_693_036,
    puid: "wikidata/61693036",
    name: "Waveform Audio",
    extensions: &["wav", "wav", "wav", "wav"],
    media_types: &["audio/wav", "audio/wave", "audio/x-pn-wav", "audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
