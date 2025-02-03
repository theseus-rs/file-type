use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852021: FileFormat = FileFormat {
    id: 105_852_021,
    source_type: SourceType::Wikidata,
    name: "SuperCollider Class",
    extensions: &["sc"],
    media_types: &[
        "application/supercollider",
        "text/plain",
        "text/supercollider",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
