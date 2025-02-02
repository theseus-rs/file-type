use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_64152987: FileFormat = FileFormat {
    id: 64_152_987,
    source_type: SourceType::Wikidata,
    name: "Crossword file format",
    extensions: &["xd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
