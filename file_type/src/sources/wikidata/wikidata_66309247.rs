use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66309247: FileFormat = FileFormat {
    id: 66_309_247,
    source_type: SourceType::Wikidata,
    name: "Access Database Runtime",
    extensions: &["accdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
