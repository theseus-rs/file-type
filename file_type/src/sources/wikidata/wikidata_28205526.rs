use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205526: FileFormat = FileFormat {
    id: 28_205_526,
    source_type: SourceType::Wikidata,
    name: "Icon library",
    extensions: &["icl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
