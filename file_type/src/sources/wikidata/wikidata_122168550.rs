use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122168550: FileFormat = FileFormat {
    id: 122_168_550,
    source_type: SourceType::Wikidata,
    name: "Proactive Password Auditor Project",
    extensions: &["hdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
