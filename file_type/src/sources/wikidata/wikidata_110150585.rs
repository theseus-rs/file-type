use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110150585: FileFormat = FileFormat {
    id: 110_150_585,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 7",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
