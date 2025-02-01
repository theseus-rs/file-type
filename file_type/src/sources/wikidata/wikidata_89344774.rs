use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_89344774: FileFormat = FileFormat {
    id: 89_344_774,
    puid: "wikidata/89344774",
    name: "QuarkXPress Document 3.1",
    extensions: &["qwd", "qxd", "qxt"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
