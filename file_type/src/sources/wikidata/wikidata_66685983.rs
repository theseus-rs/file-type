use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66685983: FileFormat = FileFormat {
    id: 66_685_983,
    source_type: SourceType::Wikidata,
    name: "OR3",
    extensions: &["or3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
