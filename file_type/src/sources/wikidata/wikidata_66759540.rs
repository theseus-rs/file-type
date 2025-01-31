use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66759540: FileFormat = FileFormat {
    id: 66_759_540,
    puid: "wikidata/66759540",
    name: "Excel 97-2003 Template",
    extensions: &["xlt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
