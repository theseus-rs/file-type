use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66663821: FileFormat = FileFormat {
    id: 66_663_821,
    source_type: SourceType::Wikidata,
    name: "Hewlett-Packard Graphics Language format",
    extensions: &["hgl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
