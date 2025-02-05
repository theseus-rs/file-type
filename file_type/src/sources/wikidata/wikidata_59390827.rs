use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59390827: FileFormat = FileFormat {
    id: 59_390_827,
    source_type: SourceType::Wikidata,
    name: "Domino XML Document Export",
    extensions: &["dxl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
