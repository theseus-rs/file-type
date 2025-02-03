use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110151085: FileFormat = FileFormat {
    id: 110_151_085,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version X2",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
