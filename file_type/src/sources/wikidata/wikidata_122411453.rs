use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122411453: FileFormat = FileFormat {
    id: 122_411_453,
    source_type: SourceType::Wikidata,
    name: "Palm Pilot Symbol File",
    extensions: &["psym"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
