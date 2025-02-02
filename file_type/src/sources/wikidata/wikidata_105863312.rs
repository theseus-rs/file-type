use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863312: FileFormat = FileFormat {
    id: 105_863_312,
    source_type: SourceType::Wikidata,
    name: "Mindjet MindManager Map",
    extensions: &["mmp"],
    media_types: &["application/vnd.mindjet.mindmanager"],
    internal_signatures: &[],
    related_formats: &[],
};
