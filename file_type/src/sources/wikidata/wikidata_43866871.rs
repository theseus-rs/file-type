use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43866871: FileFormat = FileFormat {
    id: 43_866_871,
    puid: "wikidata/43866871",
    name: "PCX, version 2",
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
