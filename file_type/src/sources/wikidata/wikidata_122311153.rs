use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122311153: FileFormat = FileFormat {
    id: 122_311_153,
    puid: "wikidata/122311153",
    name: "Open Mining Format",
    extensions: &["omf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
