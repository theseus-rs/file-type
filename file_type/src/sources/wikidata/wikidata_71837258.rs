use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71837258: FileFormat = FileFormat {
    id: 71_837_258,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Compressed Drawing file format",
    extensions: &["cdx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
