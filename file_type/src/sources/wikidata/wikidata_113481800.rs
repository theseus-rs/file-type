use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113481800: FileFormat = FileFormat {
    id: 113_481_800,
    source_type: SourceType::Wikidata,
    name: "602 Text file 1.0-1.51",
    extensions: &["602"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
