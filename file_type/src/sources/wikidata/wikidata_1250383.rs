use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1250383: FileFormat = FileFormat {
    id: 1_250_383,
    puid: "wikidata/1250383",
    name: "XYZ file format",
    extensions: &["xyz"],
    media_types: &["chemical/x-xyz"],
    internal_signatures: &[],
    related_formats: &[],
};
