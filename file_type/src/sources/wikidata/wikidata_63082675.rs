use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63082675: FileFormat = FileFormat {
    id: 63_082_675,
    source_type: SourceType::Wikidata,
    name: "Waveform Audio (WAVEFORMATEXTENSIBLE)",
    extensions: &["wav", "wave"],
    media_types: &["audio/x-wav"],
    signatures: &[],
    related_formats: &[],
};
