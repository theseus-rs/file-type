use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119975385: FileFormat = FileFormat {
    id: 119_975_385,
    source_type: SourceType::Wikidata,
    name: "Style Template",
    extensions: &["tps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
