use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_104600902: FileFormat = FileFormat {
    id: 104_600_902,
    source_type: SourceType::Wikidata,
    name: "KDB",
    extensions: &["kdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
