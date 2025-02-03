use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116418918: FileFormat = FileFormat {
    id: 116_418_918,
    source_type: SourceType::Wikidata,
    name: "Adobe Photoshop Color Table",
    extensions: &["act"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
