use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84087750: FileFormat = FileFormat {
    id: 84_087_750,
    source_type: SourceType::Wikidata,
    name: "FamilyTree Maker Database 1-4",
    extensions: &["fbk", "ftw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
