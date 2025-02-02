use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127378446: FileFormat = FileFormat {
    id: 127_378_446,
    source_type: SourceType::Wikidata,
    name: "GLSL file",
    extensions: &["glsl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
