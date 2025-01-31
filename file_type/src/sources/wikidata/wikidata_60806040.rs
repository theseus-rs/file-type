use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60806040: FileFormat = FileFormat {
    id: 60_806_040,
    puid: "wikidata/60806040",
    name: "Microsoft Excel 3.0 Worksheet (xls)",
    extensions: &["xls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
