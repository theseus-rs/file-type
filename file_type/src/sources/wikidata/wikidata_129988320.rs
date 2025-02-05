use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129988320: FileFormat = FileFormat {
    id: 129_988_320,
    source_type: SourceType::Wikidata,
    name: "JMESPath query file",
    extensions: &["jp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
