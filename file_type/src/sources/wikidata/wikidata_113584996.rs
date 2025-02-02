use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113584996: FileFormat = FileFormat {
    id: 113_584_996,
    source_type: SourceType::Wikidata,
    name: "VideoFactory Project File",
    extensions: &["vf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
