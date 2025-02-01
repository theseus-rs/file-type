use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34311506: FileFormat = FileFormat {
    id: 34_311_506,
    puid: "wikidata/34311506",
    name: "Sense8 Neutral File Format, binary variant",
    extensions: &["bff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
