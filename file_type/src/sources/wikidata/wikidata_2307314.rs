use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2307314: FileFormat = FileFormat {
    id: 2_307_314,
    source_type: SourceType::Wikidata,
    name: "Direct Access Archive",
    extensions: &["daa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
