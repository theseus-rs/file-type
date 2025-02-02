use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71264683: FileFormat = FileFormat {
    id: 71_264_683,
    source_type: SourceType::Wikidata,
    name: "Hippel module",
    extensions: &["hip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
