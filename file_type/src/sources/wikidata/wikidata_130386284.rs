use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130386284: FileFormat = FileFormat {
    id: 130_386_284,
    source_type: SourceType::Wikidata,
    name: "Nix file format",
    extensions: &["nix"],
    media_types: &["text/x-nix"],
    signatures: &[],
    related_formats: &[],
};
