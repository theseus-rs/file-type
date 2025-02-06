use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27902240: FileFormat = FileFormat {
    id: 27_902_240,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.4",
    extensions: &["adi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
