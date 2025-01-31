use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167431: FileFormat = FileFormat {
    id: 29_167_431,
    puid: "wikidata/29167431",
    name: "Notes Storage Facility",
    extensions: &["nsf", "nsf"],
    media_types: &["application/vnd.lotus-notes", "application/x-lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};
