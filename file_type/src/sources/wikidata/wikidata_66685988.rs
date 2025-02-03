use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66685988: FileFormat = FileFormat {
    id: 66_685_988,
    source_type: SourceType::Wikidata,
    name: "OR5",
    extensions: &["or5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
