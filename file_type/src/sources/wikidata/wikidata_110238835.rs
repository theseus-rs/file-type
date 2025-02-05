use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110238835: FileFormat = FileFormat {
    id: 110_238_835,
    source_type: SourceType::Wikidata,
    name: "Gorilla Scheduling",
    extensions: &["sex"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
