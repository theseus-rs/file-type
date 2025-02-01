use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863312: FileFormat = FileFormat {
    id: 105_863_312,
    puid: "wikidata/105863312",
    name: "Mindjet MindManager Map",
    extensions: &["mmp"],
    media_types: &["application/vnd.mindjet.mindmanager"],
    internal_signatures: &[],
    related_formats: &[],
};
