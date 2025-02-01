use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_94984970: FileFormat = FileFormat {
    id: 94_984_970,
    puid: "wikidata/94984970",
    name: "IGC",
    extensions: &["igc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
