use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205488: FileFormat = FileFormat {
    id: 28_205_488,
    source_type: SourceType::Wikidata,
    name: "Windows Cursor",
    extensions: &["cur"],
    media_types: &["image/x-win-bitmap"],
    internal_signatures: &[],
    related_formats: &[],
};
