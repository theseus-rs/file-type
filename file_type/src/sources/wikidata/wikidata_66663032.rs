use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66663032: FileFormat = FileFormat {
    id: 66_663_032,
    source_type: SourceType::Wikidata,
    name: "Lotus Freelance Diagram",
    extensions: &["dgm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
