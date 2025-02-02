use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113292166: FileFormat = FileFormat {
    id: 113_292_166,
    source_type: SourceType::Wikidata,
    name: "Macintosh Sound Resource",
    extensions: &["snd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
