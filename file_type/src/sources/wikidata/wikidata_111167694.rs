use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111167694: FileFormat = FileFormat {
    id: 111_167_694,
    source_type: SourceType::Wikidata,
    name: "ChemBasic file",
    extensions: &["bas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
