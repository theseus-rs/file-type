use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27902119: FileFormat = FileFormat {
    id: 27_902_119,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, ADI variant, version 3.0.2",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
