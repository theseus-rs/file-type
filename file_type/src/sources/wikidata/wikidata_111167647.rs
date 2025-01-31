use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111167647: FileFormat = FileFormat {
    id: 111_167_647,
    puid: "wikidata/111167647",
    name: "ISIS/Sketch file",
    extensions: &["skc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
