use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117866986: FileFormat = FileFormat {
    id: 117_866_986,
    source_type: SourceType::Wikidata,
    name: "American Data Tech SMARTFAX file",
    extensions: &["smf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
