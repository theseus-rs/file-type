use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66310997: FileFormat = FileFormat {
    id: 66_310_997,
    source_type: SourceType::Wikidata,
    name: "Favorite Files",
    extensions: &["mfv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
