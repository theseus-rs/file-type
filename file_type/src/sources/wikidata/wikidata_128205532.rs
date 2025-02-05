use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128205532: FileFormat = FileFormat {
    id: 128_205_532,
    source_type: SourceType::Wikidata,
    name: "UDO source code file",
    extensions: &["udo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
