use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116646759: FileFormat = FileFormat {
    id: 116_646_759,
    source_type: SourceType::Wikidata,
    name: "eXcelon Studio project",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
