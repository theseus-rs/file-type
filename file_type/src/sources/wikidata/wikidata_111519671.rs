use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111519671: FileFormat = FileFormat {
    id: 111_519_671,
    puid: "wikidata/111519671",
    name: "PageMaker template file, version 5",
    extensions: &["pt5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
