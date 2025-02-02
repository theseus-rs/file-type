use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114961072: FileFormat = FileFormat {
    id: 114_961_072,
    source_type: SourceType::Wikidata,
    name: "Writer's DreamKit 4.0 file",
    extensions: &["dsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
