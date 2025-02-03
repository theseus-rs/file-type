use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130479459: FileFormat = FileFormat {
    id: 130_479_459,
    source_type: SourceType::Wikidata,
    name: "Pony source code file",
    extensions: &["pony"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
