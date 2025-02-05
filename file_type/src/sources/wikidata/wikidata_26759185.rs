use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26759185: FileFormat = FileFormat {
    id: 26_759_185,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange Binary Format",
    extensions: &["dxb"],
    media_types: &["application/x-dxb"],
    signatures: &[],
    related_formats: &[],
};
