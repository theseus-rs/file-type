use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25339304: FileFormat = FileFormat {
    id: 25_339_304,
    source_type: SourceType::Wikidata,
    name: "Timed Text Markup Language",
    extensions: &["dxfp", "ttml", "xml"],
    media_types: &["application/ttml+xml"],
    signatures: &[],
    related_formats: &[],
};
