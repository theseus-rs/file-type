use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66134564: FileFormat = FileFormat {
    id: 66_134_564,
    source_type: SourceType::Wikidata,
    name: "Photoshop DCS 1.0",
    extensions: &["eps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
