use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118672139: FileFormat = FileFormat {
    id: 118_672_139,
    source_type: SourceType::Wikidata,
    name: "Manga Studio 1.0 Document",
    extensions: &["mpf", "msf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
