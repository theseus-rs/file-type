use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66759528: FileFormat = FileFormat {
    id: 66_759_528,
    puid: "wikidata/66759528",
    name: "Excel Binary Workbook",
    extensions: &["xlsb"],
    media_types: &["application/vnd.ms-excel.sheet.binary.macroenabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};
