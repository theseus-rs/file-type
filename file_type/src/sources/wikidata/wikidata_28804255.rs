use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28804255: FileFormat = FileFormat {
    id: 28_804_255,
    source_type: SourceType::Wikidata,
    name: "Uniform Office Spreadsheet",
    extensions: &["uos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
