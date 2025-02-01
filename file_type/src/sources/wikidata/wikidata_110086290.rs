use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110086290: FileFormat = FileFormat {
    id: 110_086_290,
    puid: "wikidata/110086290",
    name: "QuarkXPress Project 2020",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
