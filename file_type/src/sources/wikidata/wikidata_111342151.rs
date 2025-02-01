use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342151: FileFormat = FileFormat {
    id: 111_342_151,
    puid: "wikidata/111342151",
    name: "J-Phone / SmdEd mobile song",
    extensions: &["smd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
