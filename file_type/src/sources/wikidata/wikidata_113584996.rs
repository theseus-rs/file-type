use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113584996: FileFormat = FileFormat {
    id: 113_584_996,
    source_type: SourceType::Wikidata,
    name: "VideoFactory Project File",
    extensions: &["vf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
