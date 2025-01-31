use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757918: FileFormat = FileFormat {
    id: 28_757_918,
    puid: "wikidata/28757918",
    name: "Google Sheet",
    extensions: &["gsheet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
