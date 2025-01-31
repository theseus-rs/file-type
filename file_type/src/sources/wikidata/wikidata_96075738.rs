use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96075738: FileFormat = FileFormat {
    id: 96_075_738,
    puid: "wikidata/96075738",
    name: "Pajek format",
    extensions: &["net"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
