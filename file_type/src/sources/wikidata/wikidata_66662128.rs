use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66662128: FileFormat = FileFormat {
    id: 66_662_128,
    source_type: SourceType::Wikidata,
    name: "Lotus Organizer",
    extensions: &["org"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
