use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207553: FileFormat = FileFormat {
    id: 28_207_553,
    source_type: SourceType::Wikidata,
    name: "XGA",
    extensions: &["xga"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
