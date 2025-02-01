use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128034217: FileFormat = FileFormat {
    id: 128_034_217,
    puid: "wikidata/128034217",
    name: "QuarkXPress Project 18",
    extensions: &["qpt", "qwd", "qxp"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
