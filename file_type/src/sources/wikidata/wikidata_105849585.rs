use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849585: FileFormat = FileFormat {
    id: 105_849_585,
    source_type: SourceType::Wikidata,
    name: "Construct 3 Project",
    extensions: &["c3p"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
