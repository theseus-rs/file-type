use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84087750: FileFormat = FileFormat {
    id: 84_087_750,
    source_type: SourceType::Wikidata,
    name: "FamilyTree Maker Database 1-4",
    extensions: &["fbk", "ftw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
