use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124844286: FileFormat = FileFormat {
    id: 124_844_286,
    source_type: SourceType::Wikidata,
    name: "CyberLink MediaShow Project",
    extensions: &["mbp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
