use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114978471: FileFormat = FileFormat {
    id: 114_978_471,
    source_type: SourceType::Wikidata,
    name: "Hollywood Screenwriter Script File",
    extensions: &["hws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
