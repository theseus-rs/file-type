use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342229: FileFormat = FileFormat {
    id: 111_342_229,
    puid: "wikidata/111342229",
    name: "Sounder sound file",
    extensions: &["sndr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
