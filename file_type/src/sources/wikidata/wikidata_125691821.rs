use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125691821: FileFormat = FileFormat {
    id: 125_691_821,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Master Document",
    extensions: &["odm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
