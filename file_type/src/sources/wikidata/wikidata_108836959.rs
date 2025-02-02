use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108836959: FileFormat = FileFormat {
    id: 108_836_959,
    source_type: SourceType::Wikidata,
    name: "Nero ISO CD Compilation File",
    extensions: &["nri"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
