use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113083700: FileFormat = FileFormat {
    id: 113_083_700,
    source_type: SourceType::Wikidata,
    name: "Okino Transfer File Format",
    extensions: &["bdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
