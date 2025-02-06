use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207503: FileFormat = FileFormat {
    id: 28_207_503,
    source_type: SourceType::Wikidata,
    name: "WinMiPS",
    extensions: &["pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
