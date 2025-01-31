use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113292166: FileFormat = FileFormat {
    id: 113_292_166,
    puid: "wikidata/113292166",
    name: "Macintosh Sound Resource",
    extensions: &["snd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
