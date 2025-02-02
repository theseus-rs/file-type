use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28731055: FileFormat = FileFormat {
    id: 28_731_055,
    source_type: SourceType::Wikidata,
    name: "Ability Spreadsheet Template",
    extensions: &["ast"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
