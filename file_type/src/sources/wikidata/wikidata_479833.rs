use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_479833: FileFormat = FileFormat {
    id: 479_833,
    puid: "wikidata/479833",
    name: "batch file",
    extensions: &["bat", "btm", "cmd", "vbs"],
    media_types: &[
        "application/x-bat",
        "application/x-bat",
        "application/x-bat",
        "application/x-bat",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
