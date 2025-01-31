use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284556: FileFormat = FileFormat {
    id: 111_284_556,
    puid: "wikidata/111284556",
    name: "GigaStudio/GigaSampler file",
    extensions: &["gi!", "gig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
