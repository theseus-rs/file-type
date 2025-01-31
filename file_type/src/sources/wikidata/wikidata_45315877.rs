use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_45315877: FileFormat = FileFormat {
    id: 45_315_877,
    puid: "wikidata/45315877",
    name: "Macromedia Freehand file format, version 9",
    extensions: &["fh9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
