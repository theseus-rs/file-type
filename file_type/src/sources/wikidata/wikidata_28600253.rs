use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600253: FileFormat = FileFormat {
    id: 28_600_253,
    source_type: SourceType::Wikidata,
    name: ".art",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
