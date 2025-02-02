use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_56655440: FileFormat = FileFormat {
    id: 56_655_440,
    source_type: SourceType::Wikidata,
    name: "HDT",
    extensions: &["hdt"],
    media_types: &["application/vnd.hdt"],
    internal_signatures: &[],
    related_formats: &[],
};
