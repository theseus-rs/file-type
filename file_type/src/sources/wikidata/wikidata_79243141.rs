use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_79243141: FileFormat = FileFormat {
    id: 79_243_141,
    source_type: SourceType::Wikidata,
    name: "Affinity Design document",
    extensions: &["afdesign"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
