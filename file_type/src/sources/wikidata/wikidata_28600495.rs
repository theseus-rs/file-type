use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600495: FileFormat = FileFormat {
    id: 28_600_495,
    source_type: SourceType::Wikidata,
    name: "Dia",
    extensions: &["dia"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
