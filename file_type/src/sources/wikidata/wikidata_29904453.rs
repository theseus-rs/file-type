use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29904453: FileFormat = FileFormat {
    id: 29_904_453,
    source_type: SourceType::Wikidata,
    name: "PowerPacker",
    extensions: &["pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
