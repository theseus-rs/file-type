use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60873199: FileFormat = FileFormat {
    id: 60_873_199,
    source_type: SourceType::Wikidata,
    name: "Excel 95 Workbook",
    extensions: &["xls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
