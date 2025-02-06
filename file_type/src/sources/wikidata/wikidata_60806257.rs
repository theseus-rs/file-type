use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60806257: FileFormat = FileFormat {
    id: 60_806_257,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 4.0 Worksheet (xls)",
    extensions: &["xls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
