use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1258721: FileFormat = FileFormat {
    id: 1_258_721,
    source_type: SourceType::Wikidata,
    name: "NFO",
    extensions: &["nfo"],
    media_types: &["text/x-nfo"],
    internal_signatures: &[],
    related_formats: &[],
};
