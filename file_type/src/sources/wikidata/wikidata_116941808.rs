use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116941808: FileFormat = FileFormat {
    id: 116_941_808,
    puid: "wikidata/116941808",
    name: "Ulead Template Extension",
    extensions: &["tpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
