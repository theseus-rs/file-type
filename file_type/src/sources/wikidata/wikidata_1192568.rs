use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1192568: FileFormat = FileFormat {
    id: 1_192_568,
    puid: "wikidata/1192568",
    name: ".sys",
    extensions: &["sys"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
