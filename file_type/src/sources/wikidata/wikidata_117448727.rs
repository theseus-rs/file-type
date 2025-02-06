use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117448727: FileFormat = FileFormat {
    id: 117_448_727,
    source_type: SourceType::Wikidata,
    name: "Transcriber AG TAG Format",
    extensions: &["tag"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
