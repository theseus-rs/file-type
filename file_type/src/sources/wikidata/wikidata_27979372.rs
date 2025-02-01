use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979372: FileFormat = FileFormat {
    id: 27_979_372,
    puid: "wikidata/27979372",
    name: "Kate",
    extensions: &["ogx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
