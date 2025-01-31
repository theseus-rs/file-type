use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111167694: FileFormat = FileFormat {
    id: 111_167_694,
    puid: "wikidata/111167694",
    name: "ChemBasic file",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
