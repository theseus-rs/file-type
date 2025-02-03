use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206538: FileFormat = FileFormat {
    id: 28_206_538,
    source_type: SourceType::Wikidata,
    name: "Magick Persistent Cache (.cache file)",
    extensions: &["cache"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
