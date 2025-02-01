use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29465145: FileFormat = FileFormat {
    id: 29_465_145,
    puid: "wikidata/29465145",
    name: "Valve Material Type",
    extensions: &["vmt"],
    media_types: &["application/vnd.valve.source.material"],
    internal_signatures: &[],
    related_formats: &[],
};
