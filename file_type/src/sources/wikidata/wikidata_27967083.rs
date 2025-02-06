use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967083: FileFormat = FileFormat {
    id: 27_967_083,
    source_type: SourceType::Wikidata,
    name: "Digital Illusions",
    extensions: &["di"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
