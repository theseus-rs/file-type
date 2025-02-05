use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849693: FileFormat = FileFormat {
    id: 105_849_693,
    source_type: SourceType::Wikidata,
    name: "Motion Capture File Format",
    extensions: &["csm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
