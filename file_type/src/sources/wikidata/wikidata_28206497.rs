use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206497: FileFormat = FileFormat {
    id: 28_206_497,
    source_type: SourceType::Wikidata,
    name: "Lossless JPEG",
    extensions: &["jpg", "ljpeg", "ljpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
