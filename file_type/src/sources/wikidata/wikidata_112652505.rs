use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112652505: FileFormat = FileFormat {
    id: 112_652_505,
    source_type: SourceType::Wikidata,
    name: "Astound Media Library format",
    extensions: &["mml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
