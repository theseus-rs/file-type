use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_89682010: FileFormat = FileFormat {
    id: 89_682_010,
    puid: "wikidata/89682010",
    name: "QuarkXPress Document 5",
    extensions: &["qwd", "qxd", "qxt"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
