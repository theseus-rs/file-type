use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125298126: FileFormat = FileFormat {
    id: 125_298_126,
    puid: "wikidata/125298126",
    name: "Scheme library definition",
    extensions: &["sld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
