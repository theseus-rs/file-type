use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66004695: FileFormat = FileFormat {
    id: 66_004_695,
    source_type: SourceType::Wikidata,
    name: "ImageStyler File",
    extensions: &["ist"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
