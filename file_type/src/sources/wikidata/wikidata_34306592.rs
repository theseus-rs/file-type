use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34306592: FileFormat = FileFormat {
    id: 34_306_592,
    puid: "wikidata/34306592",
    name: "Scifer archive binary header",
    extensions: &["ba"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
