use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59537335: FileFormat = FileFormat {
    id: 59_537_335,
    source_type: SourceType::Wikidata,
    name: "Apple iWorks Keynote",
    extensions: &["key"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
