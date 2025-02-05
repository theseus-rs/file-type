use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48223393: FileFormat = FileFormat {
    id: 48_223_393,
    source_type: SourceType::Wikidata,
    name: "PageMaker Time Stamp File",
    extensions: &["tym"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
