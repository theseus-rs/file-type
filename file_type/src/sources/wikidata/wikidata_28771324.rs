use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771324: FileFormat = FileFormat {
    id: 28_771_324,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Spreadsheet",
    extensions: &["wps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
