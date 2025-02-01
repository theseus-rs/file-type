use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951166: FileFormat = FileFormat {
    id: 126_951_166,
    puid: "wikidata/126951166",
    name: "Groovy script file",
    extensions: &["groovy", "gsh", "gvy", "gy"],
    media_types: &[
        "text/x-groovy",
        "text/x-groovy",
        "text/x-groovy",
        "text/x-groovy",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
