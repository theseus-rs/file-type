use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110150521: FileFormat = FileFormat {
    id: 110_150_521,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 6",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
