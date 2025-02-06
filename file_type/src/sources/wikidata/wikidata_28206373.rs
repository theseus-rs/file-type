use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206373: FileFormat = FileFormat {
    id: 28_206_373,
    source_type: SourceType::Wikidata,
    name: "Interleaf image",
    extensions: &["iimg", "img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
