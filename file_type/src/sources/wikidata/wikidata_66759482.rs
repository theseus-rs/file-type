use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66759482: FileFormat = FileFormat {
    id: 66_759_482,
    puid: "wikidata/66759482",
    name: "InfoPath Form file",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
