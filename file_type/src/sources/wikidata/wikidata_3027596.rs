use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3027596: FileFormat = FileFormat {
    id: 3_027_596,
    source_type: SourceType::Wikidata,
    name: "DGN",
    extensions: &["dgn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
