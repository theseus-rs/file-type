use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440962: FileFormat = FileFormat {
    id: 111_440_962,
    puid: "wikidata/111440962",
    name: "Visual Basic UserControl Object File",
    extensions: &["ctl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
