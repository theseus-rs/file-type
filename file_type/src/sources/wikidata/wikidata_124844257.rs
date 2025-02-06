use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124844257: FileFormat = FileFormat {
    id: 124_844_257,
    source_type: SourceType::Wikidata,
    name: "MediaShow Slideshow Project File",
    extensions: &["mse"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
