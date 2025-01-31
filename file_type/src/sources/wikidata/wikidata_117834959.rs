use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117834959: FileFormat = FileFormat {
    id: 117_834_959,
    puid: "wikidata/117834959",
    name: "Brooktrout Fax-Mail 96 file",
    extensions: &["brk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
