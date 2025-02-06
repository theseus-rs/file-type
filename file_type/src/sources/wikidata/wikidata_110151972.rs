use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110151972: FileFormat = FileFormat {
    id: 110_151_972,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version X5",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
