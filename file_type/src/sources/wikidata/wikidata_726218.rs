use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_726218: FileFormat = FileFormat {
    id: 726_218,
    source_type: SourceType::Wikidata,
    name: "XML User Interface Language",
    extensions: &["xul"],
    media_types: &["application/vnd.mozilla.xul+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
