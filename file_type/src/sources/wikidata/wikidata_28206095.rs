use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206095: FileFormat = FileFormat {
    id: 28_206_095,
    source_type: SourceType::Wikidata,
    name: "Fuckpaint PI9",
    extensions: &["PI9"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
