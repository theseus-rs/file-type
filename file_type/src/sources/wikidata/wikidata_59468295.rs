use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59468295: FileFormat = FileFormat {
    id: 59_468_295,
    puid: "wikidata/59468295",
    name: "Statistical Analysis System Data XPT (Windows)",
    extensions: &["xpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
