use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111332843: FileFormat = FileFormat {
    id: 111_332_843,
    puid: "wikidata/111332843",
    name: "Roland S-7xx series floppy disk image",
    extensions: &["out", "sdk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
