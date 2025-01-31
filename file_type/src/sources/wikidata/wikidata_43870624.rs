use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43870624: FileFormat = FileFormat {
    id: 43_870_624,
    puid: "wikidata/43870624",
    name: "PCX, version 5",
    extensions: &["pcc", "pcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
