use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445591: FileFormat = FileFormat {
    id: 28_445_591,
    source_type: SourceType::Wikidata,
    name: "AMOS BASIC tokenized file",
    extensions: &["amos"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
