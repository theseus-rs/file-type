use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119823553: FileFormat = FileFormat {
    id: 119_823_553,
    puid: "wikidata/119823553",
    name: "BNTX",
    extensions: &["bntx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
