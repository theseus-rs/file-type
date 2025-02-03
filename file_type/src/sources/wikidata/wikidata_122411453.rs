use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122411453: FileFormat = FileFormat {
    id: 122_411_453,
    source_type: SourceType::Wikidata,
    name: "Palm Pilot Symbol File",
    extensions: &["psym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
