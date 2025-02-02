use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59608150: FileFormat = FileFormat {
    id: 59_608_150,
    source_type: SourceType::Wikidata,
    name: "Microsoft Expression Media",
    extensions: &["ivc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
