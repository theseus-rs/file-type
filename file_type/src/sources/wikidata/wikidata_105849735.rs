use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849735: FileFormat = FileFormat {
    id: 105_849_735,
    puid: "wikidata/105849735",
    name: "Celestia Sphere displacement Mesh (with rem)",
    extensions: &["cms"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
