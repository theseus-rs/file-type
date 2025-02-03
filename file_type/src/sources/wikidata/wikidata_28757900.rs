use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757900: FileFormat = FileFormat {
    id: 28_757_900,
    source_type: SourceType::Wikidata,
    name: "Glyph Interchange Format",
    extensions: &["glif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
