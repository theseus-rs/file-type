use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28692741: FileFormat = FileFormat {
    id: 28_692_741,
    source_type: SourceType::Wikidata,
    name: "FAV File Format",
    extensions: &["fav"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
