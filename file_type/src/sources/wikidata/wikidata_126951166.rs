use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126951166: FileFormat = FileFormat {
    id: 126_951_166,
    source_type: SourceType::Wikidata,
    name: "Groovy script file",
    extensions: &["groovy", "gsh", "gvy", "gy"],
    media_types: &["text/x-groovy"],
    internal_signatures: &[],
    related_formats: &[],
};
