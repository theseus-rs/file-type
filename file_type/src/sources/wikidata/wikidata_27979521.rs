use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979521: FileFormat = FileFormat {
    id: 27_979_521,
    source_type: SourceType::Wikidata,
    name: "Plex Video Preview Thumbnail",
    extensions: &["bif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
