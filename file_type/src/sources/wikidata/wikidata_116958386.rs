use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116958386: FileFormat = FileFormat {
    id: 116_958_386,
    source_type: SourceType::Wikidata,
    name: "Pegasus PIC",
    extensions: &["pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
