use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63082675: FileFormat = FileFormat {
    id: 63_082_675,
    source_type: SourceType::Wikidata,
    name: "Waveform Audio (WAVEFORMATEXTENSIBLE)",
    extensions: &["wav", "wave"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
