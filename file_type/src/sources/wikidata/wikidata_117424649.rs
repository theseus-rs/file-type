use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117424649: FileFormat = FileFormat {
    id: 117_424_649,
    source_type: SourceType::Wikidata,
    name: "Stationery file",
    extensions: &["sta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
