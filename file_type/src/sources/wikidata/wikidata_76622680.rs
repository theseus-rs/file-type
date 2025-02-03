use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76622680: FileFormat = FileFormat {
    id: 76_622_680,
    source_type: SourceType::Wikidata,
    name: "Turboprint Wizard",
    extensions: &["wizard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
