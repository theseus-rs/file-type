use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551294: FileFormat = FileFormat {
    id: 28_551_294,
    source_type: SourceType::Wikidata,
    name: "Adobe Color Table File",
    extensions: &["act"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
