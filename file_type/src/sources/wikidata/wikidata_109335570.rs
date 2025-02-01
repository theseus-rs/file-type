use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109335570: FileFormat = FileFormat {
    id: 109_335_570,
    puid: "wikidata/109335570",
    name: "comic book archive, ZIP container",
    extensions: &["cbz"],
    media_types: &["application/vnd.comicbook+cbz"],
    internal_signatures: &[],
    related_formats: &[],
};
