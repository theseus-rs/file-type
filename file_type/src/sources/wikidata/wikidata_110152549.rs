use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110152549: FileFormat = FileFormat {
    id: 110_152_549,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version X6",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
