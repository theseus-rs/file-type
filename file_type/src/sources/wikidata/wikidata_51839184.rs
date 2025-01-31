use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51839184: FileFormat = FileFormat {
    id: 51_839_184,
    puid: "wikidata/51839184",
    name: "Ventura Publisher",
    extensions: &["gen"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
