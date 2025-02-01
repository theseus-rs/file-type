use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_89347372: FileFormat = FileFormat {
    id: 89_347_372,
    puid: "wikidata/89347372",
    name: "QuarkXPress Document 4",
    extensions: &["qwd", "qxd", "qxt"],
    media_types: &[
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
        "application/vnd.Quark.QuarkXPress",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
