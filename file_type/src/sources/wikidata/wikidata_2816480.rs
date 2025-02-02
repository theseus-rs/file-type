use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2816480: FileFormat = FileFormat {
    id: 2_816_480,
    source_type: SourceType::Wikidata,
    name: "3DXML",
    extensions: &["3dxml"],
    media_types: &["application/x-3dxmlplugin"],
    internal_signatures: &[],
    related_formats: &[],
};
