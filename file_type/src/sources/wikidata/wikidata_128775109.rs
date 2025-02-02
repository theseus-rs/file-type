use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128775109: FileFormat = FileFormat {
    id: 128_775_109,
    source_type: SourceType::Wikidata,
    name: "Component Pascal source code file",
    extensions: &["cp"],
    media_types: &["text/x-component-pascal"],
    internal_signatures: &[],
    related_formats: &[],
};
