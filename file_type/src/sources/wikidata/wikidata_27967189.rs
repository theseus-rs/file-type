use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967189: FileFormat = FileFormat {
    id: 27_967_189,
    source_type: SourceType::Wikidata,
    name: "Fuzzac Packer module",
    extensions: &["fuz", "fuzz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
