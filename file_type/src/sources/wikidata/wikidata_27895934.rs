use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27895934: FileFormat = FileFormat {
    id: 27_895_934,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, version 1",
    extensions: &["adi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
