use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66305549: FileFormat = FileFormat {
    id: 66_305_549,
    source_type: SourceType::Wikidata,
    name: "Splitted AVI File",
    extensions: &["nvavi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
