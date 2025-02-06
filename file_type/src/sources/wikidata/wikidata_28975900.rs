use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975900: FileFormat = FileFormat {
    id: 28_975_900,
    source_type: SourceType::Wikidata,
    name: "Control surface geometry file",
    extensions: &["csf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
