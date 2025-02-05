use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_64152987: FileFormat = FileFormat {
    id: 64_152_987,
    source_type: SourceType::Wikidata,
    name: "Crossword file format",
    extensions: &["xd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
