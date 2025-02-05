use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66663053: FileFormat = FileFormat {
    id: 66_663_053,
    source_type: SourceType::Wikidata,
    name: "eSuite Presentations Graphics",
    extensions: &["pg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
