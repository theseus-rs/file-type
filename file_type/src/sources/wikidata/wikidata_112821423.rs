use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112821423: FileFormat = FileFormat {
    id: 112_821_423,
    puid: "wikidata/112821423",
    name: "Minolta 3D Scanner Element File",
    extensions: &["vvd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
