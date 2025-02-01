use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113292085: FileFormat = FileFormat {
    id: 113_292_085,
    puid: "wikidata/113292085",
    name: "InterVoice file",
    extensions: &["ivc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
