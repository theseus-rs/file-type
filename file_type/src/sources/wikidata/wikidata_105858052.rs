use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858052: FileFormat = FileFormat {
    id: 105_858_052,
    source_type: SourceType::Wikidata,
    name: "Inkscape extension descriptor",
    extensions: &["inx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
