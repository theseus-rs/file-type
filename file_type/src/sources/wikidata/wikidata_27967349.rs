use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967349: FileFormat = FileFormat {
    id: 27_967_349,
    puid: "wikidata/27967349",
    name: "iTunes Music Library, binary variant",
    extensions: &["itl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
