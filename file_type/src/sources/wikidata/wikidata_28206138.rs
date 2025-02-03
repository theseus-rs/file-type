use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206138: FileFormat = FileFormat {
    id: 28_206_138,
    source_type: SourceType::Wikidata,
    name: "Freedom of Press Info",
    extensions: &["1", "fop"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
