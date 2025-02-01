use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009551: FileFormat = FileFormat {
    id: 111_009_551,
    puid: "wikidata/111009551",
    name: "PrintMaster Craft File format",
    extensions: &["cft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
