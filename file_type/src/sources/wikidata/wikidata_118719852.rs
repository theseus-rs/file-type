use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118719852: FileFormat = FileFormat {
    id: 118_719_852,
    puid: "wikidata/118719852",
    name: "Poser 1.0 Pose Library",
    extensions: &["plb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
