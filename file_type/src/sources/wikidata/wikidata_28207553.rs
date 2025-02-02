use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207553: FileFormat = FileFormat {
    id: 28_207_553,
    source_type: SourceType::Wikidata,
    name: "XGA",
    extensions: &["xga"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
