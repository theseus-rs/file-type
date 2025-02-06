use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_945923: FileFormat = FileFormat {
    id: 945_923,
    source_type: SourceType::Wikidata,
    name: "Web application ARchive",
    extensions: &["war"],
    media_types: &["application/java-archive"],
    signatures: &[],
    related_formats: &[],
};
