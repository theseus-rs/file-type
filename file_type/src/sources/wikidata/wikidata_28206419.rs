use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206419: FileFormat = FileFormat {
    id: 28_206_419,
    puid: "wikidata/28206419",
    name: "LuraWave JPEG-2000 Code Stream Format",
    extensions: &["jpc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
