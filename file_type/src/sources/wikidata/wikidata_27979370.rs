use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979370: FileFormat = FileFormat {
    id: 27_979_370,
    source_type: SourceType::Wikidata,
    name: "EBU STL",
    extensions: &["stl"],
    media_types: &["application/x-ebu-stl"],
    internal_signatures: &[],
    related_formats: &[],
};
