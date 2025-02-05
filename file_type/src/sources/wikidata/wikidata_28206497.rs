use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206497: FileFormat = FileFormat {
    id: 28_206_497,
    source_type: SourceType::Wikidata,
    name: "Lossless JPEG",
    extensions: &["jpg", "ljpeg", "ljpg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
