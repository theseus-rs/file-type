use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111332700: FileFormat = FileFormat {
    id: 111_332_700,
    puid: "wikidata/111332700",
    name: "Roland S-5xx series floppy disk image",
    extensions: &["out", "sdk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
