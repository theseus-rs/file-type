use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856682: FileFormat = FileFormat {
    id: 105_856_682,
    source_type: SourceType::Wikidata,
    name: "Uniform Office Format (generic)",
    extensions: &["uof"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
