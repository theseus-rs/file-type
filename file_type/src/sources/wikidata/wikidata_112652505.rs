use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112652505: FileFormat = FileFormat {
    id: 112_652_505,
    source_type: SourceType::Wikidata,
    name: "Astound Media Library format",
    extensions: &["mml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
