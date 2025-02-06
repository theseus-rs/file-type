use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21652057: FileFormat = FileFormat {
    id: 21_652_057,
    source_type: SourceType::Wikidata,
    name: "Game Cache File",
    extensions: &["gcf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
