use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56315514: FileFormat = FileFormat {
    id: 56_315_514,
    puid: "wikidata/56315514",
    name: "UML diagram",
    extensions: &["uml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
