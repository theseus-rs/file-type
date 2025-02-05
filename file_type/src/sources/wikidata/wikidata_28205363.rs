use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205363: FileFormat = FileFormat {
    id: 28_205_363,
    source_type: SourceType::Wikidata,
    name: "KDC (P-Series)",
    extensions: &["kdc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
