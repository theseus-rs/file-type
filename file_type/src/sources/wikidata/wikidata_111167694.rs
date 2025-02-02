use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111167694: FileFormat = FileFormat {
    id: 111_167_694,
    source_type: SourceType::Wikidata,
    name: "ChemBasic file",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
