use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87894240: FileFormat = FileFormat {
    id: 87_894_240,
    source_type: SourceType::Wikidata,
    name: "Avery Label Pro Document 3",
    extensions: &["lpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
