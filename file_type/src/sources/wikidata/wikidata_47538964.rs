use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47538964: FileFormat = FileFormat {
    id: 47_538_964,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Source Menu File",
    extensions: &["mns"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[],
    related_formats: &[],
};
