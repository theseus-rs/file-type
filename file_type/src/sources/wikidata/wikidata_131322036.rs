use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131322036: FileFormat = FileFormat {
    id: 131_322_036,
    source_type: SourceType::Wikidata,
    name: "Treetop file format",
    extensions: &["treetop", "tt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
