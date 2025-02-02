use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60806257: FileFormat = FileFormat {
    id: 60_806_257,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 4.0 Worksheet (xls)",
    extensions: &["xls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
