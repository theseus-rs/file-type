use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127700023: FileFormat = FileFormat {
    id: 127_700_023,
    puid: "wikidata/127700023",
    name: "Gravity file",
    extensions: &["grv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
