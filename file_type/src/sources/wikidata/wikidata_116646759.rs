use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116646759: FileFormat = FileFormat {
    id: 116_646_759,
    source_type: SourceType::Wikidata,
    name: "eXcelon Studio project",
    extensions: &["prj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
