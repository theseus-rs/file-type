use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116884421: FileFormat = FileFormat {
    id: 116_884_421,
    source_type: SourceType::Wikidata,
    name: "Adobe PhotoDeluxe data",
    extensions: &["pbd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
