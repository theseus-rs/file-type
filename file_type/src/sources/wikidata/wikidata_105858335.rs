use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858335: FileFormat = FileFormat {
    id: 105_858_335,
    source_type: SourceType::Wikidata,
    name: "Adobe Edge Project",
    extensions: &["edge"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
