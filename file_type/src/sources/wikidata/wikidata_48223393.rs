use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48223393: FileFormat = FileFormat {
    id: 48_223_393,
    source_type: SourceType::Wikidata,
    name: "PageMaker Time Stamp File",
    extensions: &["tym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
