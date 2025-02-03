use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_99184084: FileFormat = FileFormat {
    id: 99_184_084,
    source_type: SourceType::Wikidata,
    name: "Atom web feed",
    extensions: &["atom", "xml"],
    media_types: &["application/atom+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
