use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_90559776: FileFormat = FileFormat {
    id: 90_559_776,
    puid: "wikidata/90559776",
    name: "QuarkXPress Project 9.1",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
