use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60806257: FileFormat = FileFormat {
    id: 60_806_257,
    puid: "wikidata/60806257",
    name: "Microsoft Excel 4.0 Worksheet (xls)",
    extensions: &["xls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
