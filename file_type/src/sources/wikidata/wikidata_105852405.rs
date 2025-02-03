use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852405: FileFormat = FileFormat {
    id: 105_852_405,
    source_type: SourceType::Wikidata,
    name: "Tim Newport-Peace's Special Use Airspace Format",
    extensions: &["sua"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
