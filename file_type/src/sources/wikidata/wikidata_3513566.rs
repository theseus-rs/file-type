use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3513566: FileFormat = FileFormat {
    id: 3_513_566,
    source_type: SourceType::Wikidata,
    name: "tab-separated values",
    extensions: &["tab", "tsv"],
    media_types: &["text/tab-separated-values"],
    signatures: &[],
    related_formats: &[],
};
