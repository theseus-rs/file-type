use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110238835: FileFormat = FileFormat {
    id: 110_238_835,
    source_type: SourceType::Wikidata,
    name: "Gorilla Scheduling",
    extensions: &["sex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
