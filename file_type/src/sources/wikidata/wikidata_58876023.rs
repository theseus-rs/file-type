use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58876023: FileFormat = FileFormat {
    id: 58_876_023,
    source_type: SourceType::Wikidata,
    name: "PowerProject",
    extensions: &["pp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
