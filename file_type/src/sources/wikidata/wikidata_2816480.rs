use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2816480: FileFormat = FileFormat {
    id: 2_816_480,
    source_type: SourceType::Wikidata,
    name: "3DXML",
    extensions: &["3dxml"],
    media_types: &["application/x-3dxmlplugin"],
    signatures: &[],
    related_formats: &[],
};
