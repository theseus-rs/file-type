use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59713856: FileFormat = FileFormat {
    id: 59_713_856,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Spreadsheet",
    extensions: &["xlr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
