use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128205532: FileFormat = FileFormat {
    id: 128_205_532,
    source_type: SourceType::Wikidata,
    name: "UDO source code file",
    extensions: &["udo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
