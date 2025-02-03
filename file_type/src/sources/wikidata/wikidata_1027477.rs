use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1027477: FileFormat = FileFormat {
    id: 1_027_477,
    source_type: SourceType::Wikidata,
    name: "Caltech Intermediate Form",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
