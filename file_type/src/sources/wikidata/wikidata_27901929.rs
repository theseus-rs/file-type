use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27901929: FileFormat = FileFormat {
    id: 27_901_929,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, version 2.2.7",
    extensions: &["adi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
