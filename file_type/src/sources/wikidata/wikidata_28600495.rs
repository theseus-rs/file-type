use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600495: FileFormat = FileFormat {
    id: 28_600_495,
    source_type: SourceType::Wikidata,
    name: "Dia",
    extensions: &["dia"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
