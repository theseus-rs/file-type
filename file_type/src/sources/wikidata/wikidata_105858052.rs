use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858052: FileFormat = FileFormat {
    id: 105_858_052,
    source_type: SourceType::Wikidata,
    name: "Inkscape extension descriptor",
    extensions: &["inx"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
