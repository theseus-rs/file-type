use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967108: FileFormat = FileFormat {
    id: 27_967_108,
    source_type: SourceType::Wikidata,
    name: "STOS memory bank",
    extensions: &["mbk", "mbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
