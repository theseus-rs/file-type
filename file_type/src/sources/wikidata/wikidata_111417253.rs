use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111417253: FileFormat = FileFormat {
    id: 111_417_253,
    source_type: SourceType::Wikidata,
    name: "Resource Script format",
    extensions: &["rc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
