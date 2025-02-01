use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111601199: FileFormat = FileFormat {
    id: 111_601_199,
    puid: "wikidata/111601199",
    name: "Adobe InDesign Document, version CC 2017",
    extensions: &["indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
