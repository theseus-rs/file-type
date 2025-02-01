use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117324987: FileFormat = FileFormat {
    id: 117_324_987,
    puid: "wikidata/117324987",
    name: "LabVIEW control",
    extensions: &["ctl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
