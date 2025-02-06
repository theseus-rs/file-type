use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66663030: FileFormat = FileFormat {
    id: 66_663_030,
    source_type: SourceType::Wikidata,
    name: "Lotus Freelance Clip Art",
    extensions: &["sym"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
