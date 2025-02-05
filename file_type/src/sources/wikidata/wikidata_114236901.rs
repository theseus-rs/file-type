use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114236901: FileFormat = FileFormat {
    id: 114_236_901,
    source_type: SourceType::Wikidata,
    name: "Browse Database format",
    extensions: &["bsc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
