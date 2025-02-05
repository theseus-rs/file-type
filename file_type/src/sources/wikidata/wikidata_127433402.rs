use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127433402: FileFormat = FileFormat {
    id: 127_433_402,
    source_type: SourceType::Wikidata,
    name: "Smalltalk Source Code",
    extensions: &["st"],
    media_types: &["text/x-smalltalk"],
    signatures: &[],
    related_formats: &[],
};
