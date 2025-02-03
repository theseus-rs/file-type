use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66663053: FileFormat = FileFormat {
    id: 66_663_053,
    source_type: SourceType::Wikidata,
    name: "eSuite Presentations Graphics",
    extensions: &["pg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
