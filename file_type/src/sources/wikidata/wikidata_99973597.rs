use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_99973597: FileFormat = FileFormat {
    id: 99_973_597,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 1.0 file format",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
