use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757953: FileFormat = FileFormat {
    id: 28_757_953,
    source_type: SourceType::Wikidata,
    name: "HGT",
    extensions: &["hgt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
