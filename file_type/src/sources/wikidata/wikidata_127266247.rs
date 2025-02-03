use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127266247: FileFormat = FileFormat {
    id: 127_266_247,
    source_type: SourceType::Wikidata,
    name: "Assembly file",
    extensions: &["eaf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
