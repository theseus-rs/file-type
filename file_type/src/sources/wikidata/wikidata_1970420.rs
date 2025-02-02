use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1970420: FileFormat = FileFormat {
    id: 1_970_420,
    source_type: SourceType::Wikidata,
    name: "Simple file verification",
    extensions: &["sfv"],
    media_types: &["text/x-sfv"],
    internal_signatures: &[],
    related_formats: &[],
};
