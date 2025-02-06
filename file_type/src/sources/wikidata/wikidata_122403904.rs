use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122403904: FileFormat = FileFormat {
    id: 122_403_904,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior Constructor Resource File",
    extensions: &["rsr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
