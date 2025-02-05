use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866645: FileFormat = FileFormat {
    id: 105_866_645,
    source_type: SourceType::Wikidata,
    name: "Maven Project Object Model",
    extensions: &["pom", "xml"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
