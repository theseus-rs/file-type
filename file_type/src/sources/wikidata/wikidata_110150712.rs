use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110150712: FileFormat = FileFormat {
    id: 110_150_712,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 8",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
