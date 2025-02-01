use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000658: FileFormat = FileFormat {
    id: 29_000_658,
    puid: "wikidata/29000658",
    name: "PTX",
    extensions: &["ptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
