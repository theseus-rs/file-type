use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27355642: FileFormat = FileFormat {
    id: 27_355_642,
    source_type: SourceType::Wikidata,
    name: "ADRG Source File",
    extensions: &["sou"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
