use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110152589: FileFormat = FileFormat {
    id: 110_152_589,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version X8",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
