use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26385770: FileFormat = FileFormat {
    id: 26_385_770,
    puid: "wikidata/26385770",
    name: "Still Picture Interchange File Format",
    extensions: &["jpeg", "jpeg", "jpg", "jpg", "spf", "spf", "spiff", "spiff"],
    media_types: &[
        "image/jpeg",
        "image/jpeg",
        "image/jpeg",
        "image/jpeg",
        "image/jpeg",
        "image/jpeg",
        "image/jpeg",
        "image/jpeg",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
