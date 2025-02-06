use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122410584: FileFormat = FileFormat {
    id: 122_410_584,
    source_type: SourceType::Wikidata,
    name: "PowerPC Symbol File",
    extensions: &["xsym"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
