use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849693: FileFormat = FileFormat {
    id: 105_849_693,
    source_type: SourceType::Wikidata,
    name: "Motion Capture File Format",
    extensions: &["csm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
