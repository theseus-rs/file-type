use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25339304: FileFormat = FileFormat {
    id: 25_339_304,
    source_type: SourceType::Wikidata,
    name: "Timed Text Markup Language",
    extensions: &["dxfp", "ttml", "xml"],
    media_types: &["application/ttml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
