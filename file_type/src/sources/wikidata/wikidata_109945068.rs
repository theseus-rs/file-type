use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109945068: FileFormat = FileFormat {
    id: 109_945_068,
    source_type: SourceType::Wikidata,
    name: "Goo Document file format",
    extensions: &["goo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
