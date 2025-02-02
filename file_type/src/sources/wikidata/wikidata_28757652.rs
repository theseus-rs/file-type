use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757652: FileFormat = FileFormat {
    id: 28_757_652,
    source_type: SourceType::Wikidata,
    name: "G64",
    extensions: &["g64"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
