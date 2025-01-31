use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966991: FileFormat = FileFormat {
    id: 27_966_991,
    puid: "wikidata/27966991",
    name: "Art & Magic",
    extensions: &["aam"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[],
    related_formats: &[],
};
