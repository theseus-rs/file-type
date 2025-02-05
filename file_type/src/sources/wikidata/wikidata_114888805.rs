use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114888805: FileFormat = FileFormat {
    id: 114_888_805,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Paper file",
    extensions: &["sdp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
