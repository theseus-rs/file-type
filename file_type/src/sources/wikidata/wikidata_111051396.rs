use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111051396: FileFormat = FileFormat {
    id: 111_051_396,
    puid: "wikidata/111051396",
    name: "WebAssembly text format",
    extensions: &["wast", "wat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
