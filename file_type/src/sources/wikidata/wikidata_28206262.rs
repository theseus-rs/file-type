use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206262: FileFormat = FileFormat {
    id: 28_206_262,
    source_type: SourceType::Wikidata,
    name: "HSI JPEG",
    extensions: &["hsi", "jpg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
