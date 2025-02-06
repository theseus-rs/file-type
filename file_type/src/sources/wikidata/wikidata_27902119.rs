use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27902119: FileFormat = FileFormat {
    id: 27_902_119,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.2",
    extensions: &["adi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
