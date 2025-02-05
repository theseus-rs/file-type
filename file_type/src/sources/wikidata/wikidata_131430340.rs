use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131430340: FileFormat = FileFormat {
    id: 131_430_340,
    source_type: SourceType::Wikidata,
    name: "X10 file format",
    extensions: &["x10"],
    media_types: &["text/x-x10"],
    signatures: &[],
    related_formats: &[],
};
