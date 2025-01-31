use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861120: FileFormat = FileFormat {
    id: 105_861_120,
    puid: "wikidata/105861120",
    name: "SyncTERM dialing directory",
    extensions: &["lst"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
