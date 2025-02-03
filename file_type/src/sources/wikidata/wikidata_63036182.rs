use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63036182: FileFormat = FileFormat {
    id: 63_036_182,
    source_type: SourceType::Wikidata,
    name: "8-bit ASCII Text",
    extensions: &["asc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
