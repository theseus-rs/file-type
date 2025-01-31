use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363669: FileFormat = FileFormat {
    id: 111_363_669,
    puid: "wikidata/111363669",
    name: "Wusikstation V3 sound file",
    extensions: &["wusiksnd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
