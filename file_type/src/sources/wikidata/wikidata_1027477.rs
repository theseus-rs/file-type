use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1027477: FileFormat = FileFormat {
    id: 1_027_477,
    source_type: SourceType::Wikidata,
    name: "Caltech Intermediate Form",
    extensions: &["cif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
