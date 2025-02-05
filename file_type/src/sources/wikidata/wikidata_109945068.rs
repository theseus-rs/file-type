use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109945068: FileFormat = FileFormat {
    id: 109_945_068,
    source_type: SourceType::Wikidata,
    name: "Goo Document file format",
    extensions: &["goo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
