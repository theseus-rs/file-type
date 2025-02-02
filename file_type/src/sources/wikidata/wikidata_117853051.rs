use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117853051: FileFormat = FileFormat {
    id: 117_853_051,
    source_type: SourceType::Wikidata,
    name: "HiJaak Draw file",
    extensions: &["pdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
