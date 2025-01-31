use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66663032: FileFormat = FileFormat {
    id: 66_663_032,
    puid: "wikidata/66663032",
    name: "Lotus Freelance Diagram",
    extensions: &["dgm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
