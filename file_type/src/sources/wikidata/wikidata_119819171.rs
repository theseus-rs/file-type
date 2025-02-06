use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119819171: FileFormat = FileFormat {
    id: 119_819_171,
    source_type: SourceType::Wikidata,
    name: "Final Draft AV Template",
    extensions: &["xavt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
