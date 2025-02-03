use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113083700: FileFormat = FileFormat {
    id: 113_083_700,
    source_type: SourceType::Wikidata,
    name: "Okino Transfer File Format",
    extensions: &["bdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
