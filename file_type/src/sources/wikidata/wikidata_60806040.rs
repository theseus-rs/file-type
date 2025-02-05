use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60806040: FileFormat = FileFormat {
    id: 60_806_040,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 3.0 Worksheet (xls)",
    extensions: &["xls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
