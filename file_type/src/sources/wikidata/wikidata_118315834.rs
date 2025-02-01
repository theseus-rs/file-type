use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118315834: FileFormat = FileFormat {
    id: 118_315_834,
    puid: "wikidata/118315834",
    name: "FotoSlate 4.0 Project",
    extensions: &["plp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
