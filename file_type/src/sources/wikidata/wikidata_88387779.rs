use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_88387779: FileFormat = FileFormat {
    id: 88_387_779,
    source_type: SourceType::Wikidata,
    name: "FamilyTree Maker Database 5-16",
    extensions: &["fbk", "ftw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
