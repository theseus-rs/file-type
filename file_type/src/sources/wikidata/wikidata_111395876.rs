use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111395876: FileFormat = FileFormat {
    id: 111_395_876,
    source_type: SourceType::Wikidata,
    name: "Konica format",
    extensions: &["kqp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
