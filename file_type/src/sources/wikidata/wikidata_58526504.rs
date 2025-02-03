use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58526504: FileFormat = FileFormat {
    id: 58_526_504,
    source_type: SourceType::Wikidata,
    name: "Enigma Binary File 3+",
    extensions: &["mus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
