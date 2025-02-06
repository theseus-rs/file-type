use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110151085: FileFormat = FileFormat {
    id: 110_151_085,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version X2",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
