use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83884461: FileFormat = FileFormat {
    id: 83_884_461,
    puid: "wikidata/83884461",
    name: "Windows Address Book",
    extensions: &["wab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
