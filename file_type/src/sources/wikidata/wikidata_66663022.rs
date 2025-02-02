use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66663022: FileFormat = FileFormat {
    id: 66_663_022,
    source_type: SourceType::Wikidata,
    name: "Lotus Freelance 2.x for OS/2",
    extensions: &["prs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
