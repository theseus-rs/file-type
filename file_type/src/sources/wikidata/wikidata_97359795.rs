use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_97359795: FileFormat = FileFormat {
    id: 97_359_795,
    source_type: SourceType::Wikidata,
    name: "AnIML",
    extensions: &["animl"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
