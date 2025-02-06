use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127785602: FileFormat = FileFormat {
    id: 127_785_602,
    source_type: SourceType::Wikidata,
    name: "MetaPost file",
    extensions: &["mp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
