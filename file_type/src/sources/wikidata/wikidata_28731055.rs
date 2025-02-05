use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28731055: FileFormat = FileFormat {
    id: 28_731_055,
    source_type: SourceType::Wikidata,
    name: "Ability Spreadsheet Template",
    extensions: &["ast"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
