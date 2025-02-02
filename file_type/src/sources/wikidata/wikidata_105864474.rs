use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864474: FileFormat = FileFormat {
    id: 105_864_474,
    source_type: SourceType::Wikidata,
    name: "PSpice Probe configuration",
    extensions: &["prb"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
