use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76622680: FileFormat = FileFormat {
    id: 76_622_680,
    source_type: SourceType::Wikidata,
    name: "Turboprint Wizard",
    extensions: &["wizard"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
