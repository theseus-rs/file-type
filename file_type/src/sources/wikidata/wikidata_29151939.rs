use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29151939: FileFormat = FileFormat {
    id: 29_151_939,
    source_type: SourceType::Wikidata,
    name: "Random Dot Software Graphic QDV",
    extensions: &["qdv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
