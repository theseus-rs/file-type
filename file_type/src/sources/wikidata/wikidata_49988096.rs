use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49988096: FileFormat = FileFormat {
    id: 49_988_096,
    source_type: SourceType::Wikidata,
    name: "Apple iBooks format",
    extensions: &["ibooks"],
    media_types: &["application/x-ibooks+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
