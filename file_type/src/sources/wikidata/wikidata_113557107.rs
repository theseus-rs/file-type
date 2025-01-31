use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113557107: FileFormat = FileFormat {
    id: 113_557_107,
    puid: "wikidata/113557107",
    name: "Virtual CD-ROM, Uncompressed",
    extensions: &["fcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
