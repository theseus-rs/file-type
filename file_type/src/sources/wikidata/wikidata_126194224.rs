use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126194224: FileFormat = FileFormat {
    id: 126_194_224,
    puid: "wikidata/126194224",
    name: "Safetensors",
    extensions: &["safetensors"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
