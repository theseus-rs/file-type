use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116958386: FileFormat = FileFormat {
    id: 116_958_386,
    source_type: SourceType::Wikidata,
    name: "Pegasus PIC",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
