use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17092932: FileFormat = FileFormat {
    id: 17_092_932,
    puid: "wikidata/17092932",
    name: "JPEG-XT",
    extensions: &["jpeg", "jpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
