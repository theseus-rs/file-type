use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109335570: FileFormat = FileFormat {
    id: 109_335_570,
    source_type: SourceType::Wikidata,
    name: "comic book archive, ZIP container",
    extensions: &["cbz"],
    media_types: &["application/vnd.comicbook+cbz"],
    signatures: &[],
    related_formats: &[],
};
