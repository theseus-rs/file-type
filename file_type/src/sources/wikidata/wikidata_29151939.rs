use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29151939: FileFormat = FileFormat {
    id: 29_151_939,
    source_type: SourceType::Wikidata,
    name: "Random Dot Software Graphic QDV",
    extensions: &["qdv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
