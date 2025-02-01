use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60886160: FileFormat = FileFormat {
    id: 60_886_160,
    puid: "wikidata/60886160",
    name: "Microsoft Excel 97 Workbook",
    extensions: &["xls", "xlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
