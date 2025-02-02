use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205537: FileFormat = FileFormat {
    id: 28_205_537,
    source_type: SourceType::Wikidata,
    name: "Micrografx Icon",
    extensions: &["icn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
