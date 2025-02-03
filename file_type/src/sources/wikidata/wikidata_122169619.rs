use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122169619: FileFormat = FileFormat {
    id: 122_169_619,
    source_type: SourceType::Wikidata,
    name: "Task Container",
    extensions: &["rtc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
