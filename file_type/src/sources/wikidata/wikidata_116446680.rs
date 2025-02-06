use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116446680: FileFormat = FileFormat {
    id: 116_446_680,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Spreadsheet",
    extensions: &["wks"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
