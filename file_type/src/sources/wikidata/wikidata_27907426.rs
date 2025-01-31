use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27907426: FileFormat = FileFormat {
    id: 27_907_426,
    puid: "wikidata/27907426",
    name: "Amiga Disk File, compressed",
    extensions: &["adz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
