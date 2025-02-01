use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43870269: FileFormat = FileFormat {
    id: 43_870_269,
    puid: "wikidata/43870269",
    name: "PCX, version 4",
    extensions: &["pcc", "pcc", "pcx", "pcx"],
    media_types: &[
        "image/vnd.zbrush.pcx",
        "image/vnd.zbrush.pcx",
        "image/x-pcx",
        "image/x-pcx",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
