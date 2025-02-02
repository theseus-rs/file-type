use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126951583: FileFormat = FileFormat {
    id: 126_951_583,
    source_type: SourceType::Wikidata,
    name: "Kotlin Source Code File",
    extensions: &["kt", "kts"],
    media_types: &["text/x-kotlin"],
    internal_signatures: &[],
    related_formats: &[],
};
