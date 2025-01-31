use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979327: FileFormat = FileFormat {
    id: 27_979_327,
    puid: "wikidata/27979327",
    name: "Advanced Function Presentation",
    extensions: &["afp"],
    media_types: &["application/x-afp"],
    internal_signatures: &[],
    related_formats: &[],
};
