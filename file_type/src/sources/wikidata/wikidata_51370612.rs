use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51370612: FileFormat = FileFormat {
    id: 51_370_612,
    puid: "wikidata/51370612",
    name: "Inkwriter/Notetaker Document",
    extensions: &["pwi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
