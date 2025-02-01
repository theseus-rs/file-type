use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60873199: FileFormat = FileFormat {
    id: 60_873_199,
    puid: "wikidata/60873199",
    name: "Excel 95 Workbook",
    extensions: &["xls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
