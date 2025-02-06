use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111285380: FileFormat = FileFormat {
    id: 111_285_380,
    source_type: SourceType::Wikidata,
    name: "Ensoniq EPS family disk image",
    extensions: &["gkh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
