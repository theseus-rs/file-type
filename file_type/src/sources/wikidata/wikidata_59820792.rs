use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59820792: FileFormat = FileFormat {
    id: 59_820_792,
    puid: "wikidata/59820792",
    name: "Corel Presentation Exchange File",
    extensions: &["cmx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
