use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60478916: FileFormat = FileFormat {
    id: 60_478_916,
    source_type: SourceType::Wikidata,
    name: "Qsplat Model",
    extensions: &["qs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
