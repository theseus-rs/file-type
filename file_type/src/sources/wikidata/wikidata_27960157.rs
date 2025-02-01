use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960157: FileFormat = FileFormat {
    id: 27_960_157,
    puid: "wikidata/27960157",
    name: "Matroska Audio",
    extensions: &["mka", "mka"],
    media_types: &["audio/matroska", "audio/x-matroska"],
    internal_signatures: &[],
    related_formats: &[],
};
