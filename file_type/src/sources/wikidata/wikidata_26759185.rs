use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26759185: FileFormat = FileFormat {
    id: 26_759_185,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange Binary Format",
    extensions: &["dxb"],
    media_types: &["application/x-dxb"],
    internal_signatures: &[],
    related_formats: &[],
};
