use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60886160: FileFormat = FileFormat {
    id: 60_886_160,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 97 Workbook",
    extensions: &["xls", "xlw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
