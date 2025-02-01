use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63082675: FileFormat = FileFormat {
    id: 63_082_675,
    puid: "wikidata/63082675",
    name: "Waveform Audio (WAVEFORMATEXTENSIBLE)",
    extensions: &["wav", "wave"],
    media_types: &["audio/x-wav", "audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
