use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52063375: FileFormat = FileFormat {
    id: 52_063_375,
    source_type: SourceType::Wikidata,
    name: "StatGraphics Data File",
    extensions: &["aws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
