use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_53844499: FileFormat = FileFormat {
    id: 53_844_499,
    puid: "wikidata/53844499",
    name: "BibTeX style file",
    extensions: &["bst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
