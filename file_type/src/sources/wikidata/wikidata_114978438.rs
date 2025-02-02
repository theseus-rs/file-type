use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114978438: FileFormat = FileFormat {
    id: 114_978_438,
    source_type: SourceType::Wikidata,
    name: "StoryView Document",
    extensions: &["syv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
