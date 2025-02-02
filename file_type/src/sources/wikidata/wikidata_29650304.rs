use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650304: FileFormat = FileFormat {
    id: 29_650_304,
    source_type: SourceType::Wikidata,
    name: "PRT scene description",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
