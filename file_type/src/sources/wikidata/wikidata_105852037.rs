use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852037: FileFormat = FileFormat {
    id: 105_852_037,
    source_type: SourceType::Wikidata,
    name: "Digital Micrograph Script",
    extensions: &["s"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
