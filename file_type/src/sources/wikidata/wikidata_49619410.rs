use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49619410: FileFormat = FileFormat {
    id: 49_619_410,
    source_type: SourceType::Wikidata,
    name: "Revit Family File",
    extensions: &["rfa"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
