use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127691331: FileFormat = FileFormat {
    id: 127_691_331,
    source_type: SourceType::Wikidata,
    name: "Dylan source code file",
    extensions: &["dylan"],
    media_types: &["text/x-dylan"],
    signatures: &[],
    related_formats: &[],
};
