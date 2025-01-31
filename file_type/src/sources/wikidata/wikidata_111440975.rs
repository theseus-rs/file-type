use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440975: FileFormat = FileFormat {
    id: 111_440_975,
    puid: "wikidata/111440975",
    name: "Visual Basic property PAGe file",
    extensions: &["pag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
