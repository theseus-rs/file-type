use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47538964: FileFormat = FileFormat {
    id: 47_538_964,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Source Menu File",
    extensions: &["mns"],
    media_types: &["application/x-autocad"],
    signatures: &[],
    related_formats: &[],
};
