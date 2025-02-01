use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167502: FileFormat = FileFormat {
    id: 29_167_502,
    puid: "wikidata/29167502",
    name: "Open Web App Manifest",
    extensions: &["webapp"],
    media_types: &["application/x-web-app-manifest+json"],
    internal_signatures: &[],
    related_formats: &[],
};
