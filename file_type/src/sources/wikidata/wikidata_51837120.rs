use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51837120: FileFormat = FileFormat {
    id: 51_837_120,
    source_type: SourceType::Wikidata,
    name: "Scitex Continuous Tone Bitmap",
    extensions: &["ct"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
