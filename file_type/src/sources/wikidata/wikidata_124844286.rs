use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124844286: FileFormat = FileFormat {
    id: 124_844_286,
    source_type: SourceType::Wikidata,
    name: "CyberLink MediaShow Project",
    extensions: &["mbp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
