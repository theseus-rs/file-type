use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_99184084: FileFormat = FileFormat {
    id: 99_184_084,
    source_type: SourceType::Wikidata,
    name: "Atom web feed",
    extensions: &["atom", "xml"],
    media_types: &["application/atom+xml"],
    signatures: &[],
    related_formats: &[],
};
