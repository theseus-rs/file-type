use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61707565: FileFormat = FileFormat {
    id: 61_707_565,
    puid: "wikidata/61707565",
    name: "Waveform Audio (WAVEFORMATEX)",
    extensions: &["wav", "wave"],
    media_types: &["audio/x-wav", "audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
