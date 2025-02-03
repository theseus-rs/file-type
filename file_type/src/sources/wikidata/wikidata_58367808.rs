use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58367808: FileFormat = FileFormat {
    id: 58_367_808,
    source_type: SourceType::Wikidata,
    name: "BSDIFF",
    extensions: &["bsdiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
