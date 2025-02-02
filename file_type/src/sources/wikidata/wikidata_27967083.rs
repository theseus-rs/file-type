use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967083: FileFormat = FileFormat {
    id: 27_967_083,
    source_type: SourceType::Wikidata,
    name: "Digital Illusions",
    extensions: &["di"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
