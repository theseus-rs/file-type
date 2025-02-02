use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66663032: FileFormat = FileFormat {
    id: 66_663_032,
    source_type: SourceType::Wikidata,
    name: "Lotus Freelance Diagram",
    extensions: &["dgm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
