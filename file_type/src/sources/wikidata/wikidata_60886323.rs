use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60886323: FileFormat = FileFormat {
    id: 60_886_323,
    puid: "wikidata/60886323",
    name: "Microsoft Excel 2000-2003 Workbook",
    extensions: &["xls", "xlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
