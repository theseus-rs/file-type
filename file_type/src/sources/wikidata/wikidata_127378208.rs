use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127378208: FileFormat = FileFormat {
    id: 127_378_208,
    source_type: SourceType::Wikidata,
    name: "FreeBASIC source code file",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
