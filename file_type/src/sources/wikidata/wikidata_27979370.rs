use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979370: FileFormat = FileFormat {
    id: 27_979_370,
    source_type: SourceType::Wikidata,
    name: "EBU STL",
    extensions: &["stl"],
    media_types: &["application/x-ebu-stl"],
    signatures: &[],
    related_formats: &[],
};
