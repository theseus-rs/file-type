use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61707565: FileFormat = FileFormat {
    id: 61_707_565,
    source_type: SourceType::Wikidata,
    name: "Waveform Audio (WAVEFORMATEX)",
    extensions: &["wav", "wave"],
    media_types: &["audio/x-wav"],
    signatures: &[],
    related_formats: &[],
};
