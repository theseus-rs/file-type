use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116804559: FileFormat = FileFormat {
    id: 116_804_559,
    source_type: SourceType::Wikidata,
    name: "TimeWiz Project File",
    extensions: &["twz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
