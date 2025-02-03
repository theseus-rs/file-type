use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113556941: FileFormat = FileFormat {
    id: 113_556_941,
    source_type: SourceType::Wikidata,
    name: "CDR-Win Image",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
