use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757880: FileFormat = FileFormat {
    id: 28_757_880,
    source_type: SourceType::Wikidata,
    name: "git packfile",
    extensions: &["pack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
