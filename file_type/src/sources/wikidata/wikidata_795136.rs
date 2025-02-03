use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_795136: FileFormat = FileFormat {
    id: 795_136,
    source_type: SourceType::Wikidata,
    name: "Alembic",
    extensions: &["abc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
