use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2207671: FileFormat = FileFormat {
    id: 2_207_671,
    puid: "wikidata/2207671",
    name: "SQX",
    extensions: &["sqx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
