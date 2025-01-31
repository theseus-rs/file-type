use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113532977: FileFormat = FileFormat {
    id: 113_532_977,
    puid: "wikidata/113532977",
    name: "Wordcraft Chapter File",
    extensions: &["001"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
