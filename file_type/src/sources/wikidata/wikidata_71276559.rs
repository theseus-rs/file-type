use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71276559: FileFormat = FileFormat {
    id: 71_276_559,
    source_type: SourceType::Wikidata,
    name: "PESIM file format",
    extensions: &["pes"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
