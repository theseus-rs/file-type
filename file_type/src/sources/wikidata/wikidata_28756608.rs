use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28756608: FileFormat = FileFormat {
    id: 28_756_608,
    source_type: SourceType::Wikidata,
    name: "FoxPro script",
    extensions: &["prg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
