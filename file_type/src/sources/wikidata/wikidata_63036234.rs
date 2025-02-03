use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63036234: FileFormat = FileFormat {
    id: 63_036_234,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 4.0 Workbook",
    extensions: &["xlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
