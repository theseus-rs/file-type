use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_901031: FileFormat = FileFormat {
    id: 901_031,
    source_type: SourceType::Wikidata,
    name: "device independent file format",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    signatures: &[],
    related_formats: &[],
};
