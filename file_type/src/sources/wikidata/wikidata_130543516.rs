use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130543516: FileFormat = FileFormat {
    id: 130_543_516,
    source_type: SourceType::Wikidata,
    name: "PyPy log file format",
    extensions: &["pypylog"],
    media_types: &["application/x-pypylog"],
    signatures: &[],
    related_formats: &[],
};
