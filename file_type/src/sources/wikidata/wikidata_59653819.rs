use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59653819: FileFormat = FileFormat {
    id: 59_653_819,
    puid: "wikidata/59653819",
    name: "Maya Binary File Format, 64 bit",
    extensions: &["mb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
