use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049547: FileFormat = FileFormat {
    id: 28_049_547,
    source_type: SourceType::Wikidata,
    name: "STAD image",
    extensions: &["pac", "seq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
