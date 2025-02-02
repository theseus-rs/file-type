use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206006: FileFormat = FileFormat {
    id: 28_206_006,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Device-dependent Uncompressed 16-bit Data",
    extensions: &["i16"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
