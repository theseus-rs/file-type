use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52063375: FileFormat = FileFormat {
    id: 52_063_375,
    source_type: SourceType::Wikidata,
    name: "StatGraphics Data File",
    extensions: &["aws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
