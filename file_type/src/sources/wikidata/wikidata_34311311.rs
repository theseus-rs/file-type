use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34311311: FileFormat = FileFormat {
    id: 34_311_311,
    puid: "wikidata/34311311",
    name: "Sense8 Neutral File Format, plain text variant",
    extensions: &["nff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
