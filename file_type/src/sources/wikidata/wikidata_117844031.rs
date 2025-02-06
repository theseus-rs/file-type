use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117844031: FileFormat = FileFormat {
    id: 117_844_031,
    source_type: SourceType::Wikidata,
    name: "Inset Systems IGF format",
    extensions: &["igf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
