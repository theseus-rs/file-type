use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66662128: FileFormat = FileFormat {
    id: 66_662_128,
    source_type: SourceType::Wikidata,
    name: "Lotus Organizer",
    extensions: &["org"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
