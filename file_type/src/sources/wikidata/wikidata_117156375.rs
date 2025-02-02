use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117156375: FileFormat = FileFormat {
    id: 117_156_375,
    source_type: SourceType::Wikidata,
    name: "Pyro data disc project file",
    extensions: &["cwd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
