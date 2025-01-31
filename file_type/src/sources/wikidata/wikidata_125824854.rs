use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125824854: FileFormat = FileFormat {
    id: 125_824_854,
    puid: "wikidata/125824854",
    name: "Tar Zipped File",
    extensions: &["taz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
