use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_45315825: FileFormat = FileFormat {
    id: 45_315_825,
    puid: "wikidata/45315825",
    name: "Macromedia Freehand file format, version 10",
    extensions: &["fh10"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
