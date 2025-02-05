use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_20965861: FileFormat = FileFormat {
    id: 20_965_861,
    source_type: SourceType::Wikidata,
    name: "Material Template Library",
    extensions: &["mtl"],
    media_types: &["model/mtl", "text/plain"],
    signatures: &[],
    related_formats: &[],
};
