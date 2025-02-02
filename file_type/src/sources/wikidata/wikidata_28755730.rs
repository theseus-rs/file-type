use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28755730: FileFormat = FileFormat {
    id: 28_755_730,
    source_type: SourceType::Wikidata,
    name: "FDB (Legacy Family Tree)",
    extensions: &["fdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
