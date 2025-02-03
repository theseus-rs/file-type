use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27901850: FileFormat = FileFormat {
    id: 27_901_850,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, version 2.1.5",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
