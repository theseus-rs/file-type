use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967189: FileFormat = FileFormat {
    id: 27_967_189,
    source_type: SourceType::Wikidata,
    name: "Fuzzac Packer module",
    extensions: &["fuz", "fuzz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
