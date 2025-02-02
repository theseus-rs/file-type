use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61707565: FileFormat = FileFormat {
    id: 61_707_565,
    source_type: SourceType::Wikidata,
    name: "Waveform Audio (WAVEFORMATEX)",
    extensions: &["wav", "wave"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
