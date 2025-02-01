use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124670600: FileFormat = FileFormat {
    id: 124_670_600,
    puid: "wikidata/124670600",
    name: "PCX, version 0",
    extensions: &["pcx"],
    media_types: &["image/x-pcx"],
    internal_signatures: &[],
    related_formats: &[],
};
