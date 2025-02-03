use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130750886: FileFormat = FileFormat {
    id: 130_750_886,
    source_type: SourceType::Wikidata,
    name: "Sed script file",
    extensions: &["sed"],
    media_types: &["text/x-sed"],
    internal_signatures: &[],
    related_formats: &[],
};
