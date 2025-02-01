use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47512572: FileFormat = FileFormat {
    id: 47_512_572,
    puid: "wikidata/47512572",
    name: "man page",
    extensions: &[
        "1", "1", "2", "2", "3", "3", "4", "4", "5", "5", "6", "6", "7", "7", "8", "8", "man",
        "man",
    ],
    media_types: &[
        "application/x-troff-man",
        "application/x-troff-man",
        "application/x-troff-man",
        "application/x-troff-man",
        "application/x-troff-man",
        "application/x-troff-man",
        "application/x-troff-man",
        "application/x-troff-man",
        "application/x-troff-man",
        "text/troff",
        "text/troff",
        "text/troff",
        "text/troff",
        "text/troff",
        "text/troff",
        "text/troff",
        "text/troff",
        "text/troff",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
