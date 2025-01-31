use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61964244: FileFormat = FileFormat {
    id: 61_964_244,
    puid: "wikidata/61964244",
    name: "pulse EKKO header file",
    extensions: &["hd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
