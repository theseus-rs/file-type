use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111262833: FileFormat = FileFormat {
    id: 111_262_833,
    puid: "wikidata/111262833",
    name: "Velvet Studio instrument",
    extensions: &["ais"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
