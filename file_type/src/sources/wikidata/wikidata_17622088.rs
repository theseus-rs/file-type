use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17622088: FileFormat = FileFormat {
    id: 17_622_088,
    source_type: SourceType::Wikidata,
    name: "Speedo",
    extensions: &["spd"],
    media_types: &["application/x-font-speedo"],
    signatures: &[],
    related_formats: &[],
};
