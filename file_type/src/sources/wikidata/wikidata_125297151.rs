use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125297151: FileFormat = FileFormat {
    id: 125_297_151,
    source_type: SourceType::Wikidata,
    name: "cdb format",
    extensions: &["cdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
