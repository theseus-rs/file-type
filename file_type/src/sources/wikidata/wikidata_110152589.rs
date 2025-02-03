use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110152589: FileFormat = FileFormat {
    id: 110_152_589,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version X8",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
