use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127327975: FileFormat = FileFormat {
    id: 127_327_975,
    puid: "wikidata/127327975",
    name: "CUDA file",
    extensions: &["cu"],
    media_types: &["text/x-cuda"],
    internal_signatures: &[],
    related_formats: &[],
};
