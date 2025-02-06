use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130266833: FileFormat = FileFormat {
    id: 130_266_833,
    source_type: SourceType::Wikidata,
    name: "Macaulay2 file format",
    extensions: &["m2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
