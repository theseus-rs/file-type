use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114238261: FileFormat = FileFormat {
    id: 114_238_261,
    source_type: SourceType::Wikidata,
    name: "Streaming Audio Metafile",
    extensions: &["lam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
