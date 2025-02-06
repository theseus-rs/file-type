use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60873199: FileFormat = FileFormat {
    id: 60_873_199,
    source_type: SourceType::Wikidata,
    name: "Excel 95 Workbook",
    extensions: &["xls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
