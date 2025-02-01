use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122025500: FileFormat = FileFormat {
    id: 122_025_500,
    puid: "wikidata/122025500",
    name: "Scorch web page",
    extensions: &["htm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
