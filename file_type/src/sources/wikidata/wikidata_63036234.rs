use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63036234: FileFormat = FileFormat {
    id: 63_036_234,
    puid: "wikidata/63036234",
    name: "Microsoft Excel 4.0 Workbook",
    extensions: &["xlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
