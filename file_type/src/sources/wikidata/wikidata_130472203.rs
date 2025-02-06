use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130472203: FileFormat = FileFormat {
    id: 130_472_203,
    source_type: SourceType::Wikidata,
    name: "Phix file",
    extensions: &["exw"],
    media_types: &["text/x-phix"],
    signatures: &[],
    related_formats: &[],
};
