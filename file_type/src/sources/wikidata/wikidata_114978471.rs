use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114978471: FileFormat = FileFormat {
    id: 114_978_471,
    source_type: SourceType::Wikidata,
    name: "Hollywood Screenwriter Script File",
    extensions: &["hws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
