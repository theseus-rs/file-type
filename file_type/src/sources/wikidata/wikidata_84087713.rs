use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84087713: FileFormat = FileFormat {
    id: 84_087_713,
    source_type: SourceType::Wikidata,
    name: "RootsMagic Database",
    extensions: &["rmgc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
