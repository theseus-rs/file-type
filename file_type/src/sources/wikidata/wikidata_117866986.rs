use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117866986: FileFormat = FileFormat {
    id: 117_866_986,
    source_type: SourceType::Wikidata,
    name: "American Data Tech SMARTFAX file",
    extensions: &["smf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
