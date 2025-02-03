use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852054: FileFormat = FileFormat {
    id: 105_852_054,
    source_type: SourceType::Wikidata,
    name: "atari++ state",
    extensions: &["state"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
