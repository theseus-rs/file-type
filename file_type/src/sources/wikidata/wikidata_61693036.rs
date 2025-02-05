use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61693036: FileFormat = FileFormat {
    id: 61_693_036,
    source_type: SourceType::Wikidata,
    name: "Waveform Audio",
    extensions: &["wav"],
    media_types: &["audio/wav", "audio/wave", "audio/x-pn-wav", "audio/x-wav"],
    signatures: &[],
    related_formats: &[],
};
