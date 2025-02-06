use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59537335: FileFormat = FileFormat {
    id: 59_537_335,
    source_type: SourceType::Wikidata,
    name: "Apple iWorks Keynote",
    extensions: &["key"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
