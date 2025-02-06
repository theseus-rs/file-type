use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117448593: FileFormat = FileFormat {
    id: 117_448_593,
    source_type: SourceType::Wikidata,
    name: "FLExText Interlinear XML Format",
    extensions: &["flextext"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
