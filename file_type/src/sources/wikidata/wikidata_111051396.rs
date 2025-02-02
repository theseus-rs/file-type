use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111051396: FileFormat = FileFormat {
    id: 111_051_396,
    source_type: SourceType::Wikidata,
    name: "WebAssembly text format",
    extensions: &["wast", "wat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
