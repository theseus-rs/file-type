use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967518: FileFormat = FileFormat {
    id: 27_967_518,
    puid: "wikidata/27967518",
    name: "Matroska Subtitles",
    extensions: &["mks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
