use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111283602: FileFormat = FileFormat {
    id: 111_283_602,
    source_type: SourceType::Wikidata,
    name: "Casio FZ-1 full dump format",
    extensions: &["fzf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
