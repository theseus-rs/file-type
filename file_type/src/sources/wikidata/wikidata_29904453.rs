use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904453: FileFormat = FileFormat {
    id: 29_904_453,
    source_type: SourceType::Wikidata,
    name: "PowerPacker",
    extensions: &["pp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
