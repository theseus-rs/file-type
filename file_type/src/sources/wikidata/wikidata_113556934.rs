use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113556934: FileFormat = FileFormat {
    id: 113_556_934,
    puid: "wikidata/113556934",
    name: "BlindRead ImageCreator Image",
    extensions: &["iso"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
