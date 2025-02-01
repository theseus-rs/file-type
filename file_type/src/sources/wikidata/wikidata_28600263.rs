use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600263: FileFormat = FileFormat {
    id: 28_600_263,
    puid: "wikidata/28600263",
    name: "Ability Database",
    extensions: &["adb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
