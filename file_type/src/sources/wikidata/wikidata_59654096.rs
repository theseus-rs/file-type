use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59654096: FileFormat = FileFormat {
    id: 59_654_096,
    source_type: SourceType::Wikidata,
    name: "Adobe Content Server Message File",
    extensions: &["acsm"],
    media_types: &["application/vnd.adobe.adept+xml"],
    signatures: &[],
    related_formats: &[],
};
