use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48694183: FileFormat = FileFormat {
    id: 48_694_183,
    source_type: SourceType::Wikidata,
    name: "WordStar for MS-DOS Document, version 7",
    extensions: &["ws", "ws7"],
    media_types: &["application/x-wordstar"],
    internal_signatures: &[],
    related_formats: &[],
};
