use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538964: FileFormat = FileFormat {
    id: 47_538_964,
    puid: "wikidata/47538964",
    name: "AutoCAD Source Menu File",
    extensions: &["mns"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[],
    related_formats: &[],
};
