use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122169695: FileFormat = FileFormat {
    id: 122_169_695,
    source_type: SourceType::Wikidata,
    name: "Key Cache File",
    extensions: &["ekc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
