use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967077: FileFormat = FileFormat {
    id: 27_967_077,
    source_type: SourceType::Wikidata,
    name: "Beathoven Synthesiser",
    extensions: &["bss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
