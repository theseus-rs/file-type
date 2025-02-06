use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27901853: FileFormat = FileFormat {
    id: 27_901_853,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, version 2.1.7",
    extensions: &["adi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
