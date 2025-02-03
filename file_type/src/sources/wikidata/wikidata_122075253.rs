use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122075253: FileFormat = FileFormat {
    id: 122_075_253,
    source_type: SourceType::Wikidata,
    name: "Finale Template File",
    extensions: &["ftm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
