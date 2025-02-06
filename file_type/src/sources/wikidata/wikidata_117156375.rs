use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117156375: FileFormat = FileFormat {
    id: 117_156_375,
    source_type: SourceType::Wikidata,
    name: "Pyro data disc project file",
    extensions: &["cwd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
