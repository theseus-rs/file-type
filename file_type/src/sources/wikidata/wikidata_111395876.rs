use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111395876: FileFormat = FileFormat {
    id: 111_395_876,
    source_type: SourceType::Wikidata,
    name: "Konica format",
    extensions: &["kqp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
