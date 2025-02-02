use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2609791: FileFormat = FileFormat {
    id: 2_609_791,
    source_type: SourceType::Wikidata,
    name: "Blu-ray Disc Audio-Video MPEG-2 Transport Stream container file format",
    extensions: &["MTS", "m2ts"],
    media_types: &["video/MP2T"],
    internal_signatures: &[],
    related_formats: &[],
};
