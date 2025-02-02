use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84087713: FileFormat = FileFormat {
    id: 84_087_713,
    source_type: SourceType::Wikidata,
    name: "RootsMagic Database",
    extensions: &["rmgc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
