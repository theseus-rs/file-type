use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28009488: FileFormat = FileFormat {
    id: 28_009_488,
    source_type: SourceType::Wikidata,
    name: "Tibia map file",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
