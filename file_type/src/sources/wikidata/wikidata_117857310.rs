use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117857310: FileFormat = FileFormat {
    id: 117_857_310,
    puid: "wikidata/117857310",
    name: "IBM Storyboard PIC file",
    extensions: &["sbp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
