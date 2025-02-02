use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60806040: FileFormat = FileFormat {
    id: 60_806_040,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 3.0 Worksheet (xls)",
    extensions: &["xls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
