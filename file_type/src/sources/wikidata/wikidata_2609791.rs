use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2609791: FileFormat = FileFormat {
    id: 2_609_791,
    puid: "wikidata/2609791",
    name: "Blu-ray Disc Audio-Video MPEG-2 Transport Stream container file format",
    extensions: &["MTS", "m2ts"],
    media_types: &["video/MP2T", "video/MP2T"],
    internal_signatures: &[],
    related_formats: &[],
};
