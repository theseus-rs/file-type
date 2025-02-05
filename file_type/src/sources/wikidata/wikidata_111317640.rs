use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317640: FileFormat = FileFormat {
    id: 111_317_640,
    source_type: SourceType::Wikidata,
    name: "MFi - i-Melody - Melody Format for i-Mode",
    extensions: &["mld"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
