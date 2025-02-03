use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111431001: FileFormat = FileFormat {
    id: 111_431_001,
    source_type: SourceType::Wikidata,
    name: "ExtendScript Included Script File",
    extensions: &["jsxinc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
