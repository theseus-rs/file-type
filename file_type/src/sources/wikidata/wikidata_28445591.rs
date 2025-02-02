use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445591: FileFormat = FileFormat {
    id: 28_445_591,
    source_type: SourceType::Wikidata,
    name: "AMOS BASIC tokenized file",
    extensions: &["amos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
