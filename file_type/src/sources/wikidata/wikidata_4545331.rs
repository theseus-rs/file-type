use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4545331: FileFormat = FileFormat {
    id: 4_545_331,
    source_type: SourceType::Wikidata,
    name: ".3ds",
    extensions: &["3ds"],
    media_types: &["application/x-3ds", "image/x-3ds"],
    internal_signatures: &[],
    related_formats: &[],
};
