use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129988320: FileFormat = FileFormat {
    id: 129_988_320,
    source_type: SourceType::Wikidata,
    name: "JMESPath query file",
    extensions: &["jp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
