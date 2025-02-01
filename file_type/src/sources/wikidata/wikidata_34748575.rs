use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34748575: FileFormat = FileFormat {
    id: 34_748_575,
    puid: "wikidata/34748575",
    name: "Thermo-Calc Database Format",
    extensions: &["tdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
