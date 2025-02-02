use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111166091: FileFormat = FileFormat {
    id: 111_166_091,
    source_type: SourceType::Wikidata,
    name: "Ludwig song file",
    extensions: &["ludwig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
