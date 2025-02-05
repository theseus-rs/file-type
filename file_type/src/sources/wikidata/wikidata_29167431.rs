use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167431: FileFormat = FileFormat {
    id: 29_167_431,
    source_type: SourceType::Wikidata,
    name: "Notes Storage Facility",
    extensions: &["nsf"],
    media_types: &["application/vnd.lotus-notes", "application/x-lotus-notes"],
    signatures: &[],
    related_formats: &[],
};
