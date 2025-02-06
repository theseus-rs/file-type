use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66759537: FileFormat = FileFormat {
    id: 66_759_537,
    source_type: SourceType::Wikidata,
    name: "Excel Template",
    extensions: &["xltx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
