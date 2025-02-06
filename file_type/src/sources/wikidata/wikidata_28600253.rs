use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600253: FileFormat = FileFormat {
    id: 28_600_253,
    source_type: SourceType::Wikidata,
    name: ".art",
    extensions: &["art"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
