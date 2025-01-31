use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113521033: FileFormat = FileFormat {
    id: 113_521_033,
    puid: "wikidata/113521033",
    name: "BIM Metadata File",
    extensions: &["bim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
