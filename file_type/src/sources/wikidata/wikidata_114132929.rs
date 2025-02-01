use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114132929: FileFormat = FileFormat {
    id: 114_132_929,
    puid: "wikidata/114132929",
    name: "Constraint File Format",
    extensions: &["con"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
