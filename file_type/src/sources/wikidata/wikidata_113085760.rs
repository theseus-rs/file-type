use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113085760: FileFormat = FileFormat {
    id: 113_085_760,
    puid: "wikidata/113085760",
    name: "CB7",
    extensions: &["cb7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
