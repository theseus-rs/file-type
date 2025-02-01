use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3513566: FileFormat = FileFormat {
    id: 3_513_566,
    puid: "wikidata/3513566",
    name: "tab-separated values",
    extensions: &["tab", "tsv"],
    media_types: &["text/tab-separated-values", "text/tab-separated-values"],
    internal_signatures: &[],
    related_formats: &[],
};
