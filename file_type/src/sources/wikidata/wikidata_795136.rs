use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_795136: FileFormat = FileFormat {
    id: 795_136,
    source_type: SourceType::Wikidata,
    name: "Alembic",
    extensions: &["abc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
