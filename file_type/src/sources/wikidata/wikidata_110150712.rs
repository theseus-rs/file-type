use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110150712: FileFormat = FileFormat {
    id: 110_150_712,
    source_type: SourceType::Wikidata,
    name: "Serif DrawPlus Drawing, version 8",
    extensions: &["dpa", "dpp", "dpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
