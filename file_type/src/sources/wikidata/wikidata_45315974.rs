use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_45315974: FileFormat = FileFormat {
    id: 45_315_974,
    puid: "wikidata/45315974",
    name: "Macromedia Freehand MX file format, version 11",
    extensions: &["fh11"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
