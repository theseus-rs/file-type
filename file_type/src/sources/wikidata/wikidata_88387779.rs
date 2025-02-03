use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_88387779: FileFormat = FileFormat {
    id: 88_387_779,
    source_type: SourceType::Wikidata,
    name: "FamilyTree Maker Database 5-16",
    extensions: &["fbk", "ftw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
