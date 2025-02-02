use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967077: FileFormat = FileFormat {
    id: 27_967_077,
    source_type: SourceType::Wikidata,
    name: "Beathoven Synthesiser",
    extensions: &["bss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
