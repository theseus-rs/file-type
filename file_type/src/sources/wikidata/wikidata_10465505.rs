use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10465505: FileFormat = FileFormat {
    id: 10_465_505,
    source_type: SourceType::Wikidata,
    name: "DTS-HD",
    extensions: &["dtshd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
