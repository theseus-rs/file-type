use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317150: FileFormat = FileFormat {
    id: 111_317_150,
    source_type: SourceType::Wikidata,
    name: "Korg T-series wave",
    extensions: &["kft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
