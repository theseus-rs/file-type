use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66759528: FileFormat = FileFormat {
    id: 66_759_528,
    source_type: SourceType::Wikidata,
    name: "Excel Binary Workbook",
    extensions: &["xlsb"],
    media_types: &["application/vnd.ms-excel.sheet.binary.macroenabled.12"],
    signatures: &[],
    related_formats: &[],
};
