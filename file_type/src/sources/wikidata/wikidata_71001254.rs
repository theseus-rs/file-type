use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71001254: FileFormat = FileFormat {
    id: 71_001_254,
    source_type: SourceType::Wikidata,
    name: "Gameboy Sound Format",
    extensions: &["gsf", "gsflib", "minigsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
