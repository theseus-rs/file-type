use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117424649: FileFormat = FileFormat {
    id: 117_424_649,
    source_type: SourceType::Wikidata,
    name: "Stationery file",
    extensions: &["sta"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
