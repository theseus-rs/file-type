use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48693254: FileFormat = FileFormat {
    id: 48_693_254,
    source_type: SourceType::Wikidata,
    name: "WordStar for MSDOS Document, version 4",
    extensions: &["ws", "ws4"],
    media_types: &["application/x-wordstar"],
    internal_signatures: &[],
    related_formats: &[],
};
