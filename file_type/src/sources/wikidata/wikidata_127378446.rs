use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127378446: FileFormat = FileFormat {
    id: 127_378_446,
    source_type: SourceType::Wikidata,
    name: "GLSL file",
    extensions: &["glsl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
