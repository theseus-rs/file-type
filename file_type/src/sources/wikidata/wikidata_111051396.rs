use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111051396: FileFormat = FileFormat {
    id: 111_051_396,
    source_type: SourceType::Wikidata,
    name: "WebAssembly text format",
    extensions: &["wast", "wat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
