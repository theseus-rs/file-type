use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_795136: FileFormat = FileFormat {
    id: 795_136,
    puid: "wikidata/795136",
    name: "Alembic",
    extensions: &["abc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
