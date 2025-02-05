use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855536: FileFormat = FileFormat {
    id: 105_855_536,
    source_type: SourceType::Wikidata,
    name: "OpenColorIO profile (with rem)",
    extensions: &["ocio"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
