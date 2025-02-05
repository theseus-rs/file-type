use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111332298: FileFormat = FileFormat {
    id: 111_332_298,
    source_type: SourceType::Wikidata,
    name: "Typhoon voice file",
    extensions: &["o01"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
