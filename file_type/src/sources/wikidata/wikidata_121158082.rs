use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121158082: FileFormat = FileFormat {
    id: 121_158_082,
    puid: "wikidata/121158082",
    name: "ResumeMaker file",
    extensions: &["rmr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
