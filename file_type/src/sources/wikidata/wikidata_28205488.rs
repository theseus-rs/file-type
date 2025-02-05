use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205488: FileFormat = FileFormat {
    id: 28_205_488,
    source_type: SourceType::Wikidata,
    name: "Windows Cursor",
    extensions: &["cur"],
    media_types: &["image/x-win-bitmap"],
    signatures: &[],
    related_formats: &[],
};
