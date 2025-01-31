use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967091: FileFormat = FileFormat {
    id: 27_967_091,
    puid: "wikidata/27967091",
    name: "Funcom ISS",
    extensions: &["iss", "xarc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
