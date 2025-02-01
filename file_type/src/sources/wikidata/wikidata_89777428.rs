use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_89777428: FileFormat = FileFormat {
    id: 89_777_428,
    puid: "wikidata/89777428",
    name: "QuarkXPress Project 7",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
