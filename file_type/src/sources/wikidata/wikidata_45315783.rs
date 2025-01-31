use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_45315783: FileFormat = FileFormat {
    id: 45_315_783,
    puid: "wikidata/45315783",
    name: "Macromedia Freehand file format, version 5",
    extensions: &["fh5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
