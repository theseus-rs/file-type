use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130386284: FileFormat = FileFormat {
    id: 130_386_284,
    source_type: SourceType::Wikidata,
    name: "Nix file format",
    extensions: &["nix"],
    media_types: &["text/x-nix"],
    internal_signatures: &[],
    related_formats: &[],
};
