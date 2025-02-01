use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99851769: FileFormat = FileFormat {
    id: 99_851_769,
    puid: "wikidata/99851769",
    name: "ESRI Published Map Format",
    extensions: &["pmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
