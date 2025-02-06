use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47520788: FileFormat = FileFormat {
    id: 47_520_788,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version 10",
    extensions: &["ppp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
