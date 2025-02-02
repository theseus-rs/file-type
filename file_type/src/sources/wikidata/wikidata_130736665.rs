use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130736665: FileFormat = FileFormat {
    id: 130_736_665,
    source_type: SourceType::Wikidata,
    name: "Savi source code file",
    extensions: &["savi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
