use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_10465505: FileFormat = FileFormat {
    id: 10_465_505,
    source_type: SourceType::Wikidata,
    name: "DTS-HD",
    extensions: &["dtshd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
