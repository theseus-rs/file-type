use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110442377: FileFormat = FileFormat {
    id: 110_442_377,
    puid: "wikidata/110442377",
    name: "Daisy-Dot Font File, version III",
    extensions: &["nlq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
