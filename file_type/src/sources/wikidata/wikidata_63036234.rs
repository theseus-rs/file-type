use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63036234: FileFormat = FileFormat {
    id: 63_036_234,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 4.0 Workbook",
    extensions: &["xlw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
