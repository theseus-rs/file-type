use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71837258: FileFormat = FileFormat {
    id: 71_837_258,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Compressed Drawing file format",
    extensions: &["cdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
