use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127378208: FileFormat = FileFormat {
    id: 127_378_208,
    source_type: SourceType::Wikidata,
    name: "FreeBASIC source code file",
    extensions: &["bas"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
