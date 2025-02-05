use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959943: FileFormat = FileFormat {
    id: 27_959_943,
    source_type: SourceType::Wikidata,
    name: "La Lossless Audio",
    extensions: &["la"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
