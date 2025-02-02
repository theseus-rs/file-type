use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127785602: FileFormat = FileFormat {
    id: 127_785_602,
    source_type: SourceType::Wikidata,
    name: "MetaPost file",
    extensions: &["mp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
