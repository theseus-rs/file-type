use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27901855: FileFormat = FileFormat {
    id: 27_901_855,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, version 2.1.8",
    extensions: &["adi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
