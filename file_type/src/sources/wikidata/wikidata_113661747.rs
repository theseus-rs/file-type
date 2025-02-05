use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113661747: FileFormat = FileFormat {
    id: 113_661_747,
    source_type: SourceType::Wikidata,
    name: "SciFax file",
    extensions: &["sci"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
