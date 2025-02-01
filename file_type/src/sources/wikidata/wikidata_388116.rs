use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_388116: FileFormat = FileFormat {
    id: 388_116,
    puid: "wikidata/388116",
    name: "Electronic Design Interchange Format",
    extensions: &["edf", "edi", "edn", "edo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
