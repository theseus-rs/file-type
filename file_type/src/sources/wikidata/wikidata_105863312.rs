use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863312: FileFormat = FileFormat {
    id: 105_863_312,
    source_type: SourceType::Wikidata,
    name: "Mindjet MindManager Map",
    extensions: &["mmp"],
    media_types: &["application/vnd.mindjet.mindmanager"],
    signatures: &[],
    related_formats: &[],
};
