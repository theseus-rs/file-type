use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_64152987: FileFormat = FileFormat {
    id: 64_152_987,
    puid: "wikidata/64152987",
    name: "Crossword file format",
    extensions: &["xd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
