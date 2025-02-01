use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757779: FileFormat = FileFormat {
    id: 28_757_779,
    puid: "wikidata/28757779",
    name: "GME",
    extensions: &["gme"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
