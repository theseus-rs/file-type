use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117324972: FileFormat = FileFormat {
    id: 117_324_972,
    puid: "wikidata/117324972",
    name: "LabVIEW virtual instrument template",
    extensions: &["vit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
