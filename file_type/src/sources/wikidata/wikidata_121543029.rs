use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121543029: FileFormat = FileFormat {
    id: 121_543_029,
    puid: "wikidata/121543029",
    name: "DeductionPro 2008 Data file",
    extensions: &["d08"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
