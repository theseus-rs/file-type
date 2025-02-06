use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117842943: FileFormat = FileFormat {
    id: 117_842_943,
    source_type: SourceType::Wikidata,
    name: "Everex Everfax 24/96",
    extensions: &["ef3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
