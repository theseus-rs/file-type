use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2127640: FileFormat = FileFormat {
    id: 2_127_640,
    puid: "wikidata/2127640",
    name: "Railway Markup Language",
    extensions: &[
        "railml", "railml", "railml", "railmlx", "railmlx", "railmlx",
    ],
    media_types: &[
        "application/xml",
        "application/xml",
        "application/zip",
        "application/zip",
        "text/xml",
        "text/xml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
