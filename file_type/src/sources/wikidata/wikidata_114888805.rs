use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114888805: FileFormat = FileFormat {
    id: 114_888_805,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Paper file",
    extensions: &["sdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
