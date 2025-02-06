use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967091: FileFormat = FileFormat {
    id: 27_967_091,
    source_type: SourceType::Wikidata,
    name: "Funcom ISS",
    extensions: &["iss", "xarc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
