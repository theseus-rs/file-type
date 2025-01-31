use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113841104: FileFormat = FileFormat {
    id: 113_841_104,
    puid: "wikidata/113841104",
    name: "Medi@Show Film File",
    extensions: &["flm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
