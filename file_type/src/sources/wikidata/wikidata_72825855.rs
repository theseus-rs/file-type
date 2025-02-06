use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72825855: FileFormat = FileFormat {
    id: 72_825_855,
    source_type: SourceType::Wikidata,
    name: "OpenCanvas Image",
    extensions: &["oci"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
