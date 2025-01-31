use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_89897874: FileFormat = FileFormat {
    id: 89_897_874,
    puid: "wikidata/89897874",
    name: "QuarkXPress Project 8",
    extensions: &["qwd", "qxd", "qxt"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
