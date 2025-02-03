use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849585: FileFormat = FileFormat {
    id: 105_849_585,
    source_type: SourceType::Wikidata,
    name: "Construct 3 Project",
    extensions: &["c3p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
