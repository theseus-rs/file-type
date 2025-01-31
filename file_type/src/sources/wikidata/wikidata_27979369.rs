use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979369: FileFormat = FileFormat {
    id: 27_979_369,
    puid: "wikidata/27979369",
    name: "ReSample",
    extensions: &["srs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
