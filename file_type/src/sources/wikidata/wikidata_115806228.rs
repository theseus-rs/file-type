use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115806228: FileFormat = FileFormat {
    id: 115_806_228,
    source_type: SourceType::Wikidata,
    name: "JWCC",
    extensions: &["jwcc"],
    media_types: &["application/jwcc"],
    internal_signatures: &[],
    related_formats: &[],
};
