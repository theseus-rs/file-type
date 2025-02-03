use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_945923: FileFormat = FileFormat {
    id: 945_923,
    source_type: SourceType::Wikidata,
    name: "Web application ARchive",
    extensions: &["war"],
    media_types: &["application/java-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
