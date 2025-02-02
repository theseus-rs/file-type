use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849735: FileFormat = FileFormat {
    id: 105_849_735,
    source_type: SourceType::Wikidata,
    name: "Celestia Sphere displacement Mesh (with rem)",
    extensions: &["cms"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
