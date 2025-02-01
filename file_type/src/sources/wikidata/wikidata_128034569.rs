use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128034569: FileFormat = FileFormat {
    id: 128_034_569,
    puid: "wikidata/128034569",
    name: "QuarkXPress Project 19",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
