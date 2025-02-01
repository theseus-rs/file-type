use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34305583: FileFormat = FileFormat {
    id: 34_305_583,
    puid: "wikidata/34305583",
    name: "Racket script",
    extensions: &["rkt", "rkt", "rktd", "rktd", "rktl", "rktl"],
    media_types: &[
        "application/x-racket",
        "application/x-racket",
        "application/x-racket",
        "text/x-racket",
        "text/x-racket",
        "text/x-racket",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
