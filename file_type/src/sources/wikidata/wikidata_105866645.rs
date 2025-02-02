use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866645: FileFormat = FileFormat {
    id: 105_866_645,
    source_type: SourceType::Wikidata,
    name: "Maven Project Object Model",
    extensions: &["pom", "xml"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
