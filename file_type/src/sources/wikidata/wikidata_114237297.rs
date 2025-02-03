use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114237297: FileFormat = FileFormat {
    id: 114_237_297,
    source_type: SourceType::Wikidata,
    name: "Visual C++ Project file",
    extensions: &["mak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
