use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130542392: FileFormat = FileFormat {
    id: 130_542_392,
    source_type: SourceType::Wikidata,
    name: "Parallel Thread Execution file format",
    extensions: &["ptx"],
    media_types: &["text/x-ptx"],
    signatures: &[],
    related_formats: &[],
};
