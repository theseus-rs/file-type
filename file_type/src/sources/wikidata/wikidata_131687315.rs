use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131687315: FileFormat = FileFormat {
    id: 131_687_315,
    source_type: SourceType::Wikidata,
    name: "HTML Email Markup Language format",
    extensions: &["heml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
