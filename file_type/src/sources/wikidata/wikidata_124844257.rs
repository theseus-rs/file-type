use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124844257: FileFormat = FileFormat {
    id: 124_844_257,
    source_type: SourceType::Wikidata,
    name: "MediaShow Slideshow Project File",
    extensions: &["mse"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
