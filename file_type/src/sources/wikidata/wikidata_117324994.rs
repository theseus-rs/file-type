use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117324994: FileFormat = FileFormat {
    id: 117_324_994,
    puid: "wikidata/117324994",
    name: "LabVIEW control template",
    extensions: &["ctt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
