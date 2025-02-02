use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857152: FileFormat = FileFormat {
    id: 105_857_152,
    source_type: SourceType::Wikidata,
    name: "MAME Hash",
    extensions: &["hsi"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
