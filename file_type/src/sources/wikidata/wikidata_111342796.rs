use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342796: FileFormat = FileFormat {
    id: 111_342_796,
    source_type: SourceType::Wikidata,
    name: "Roland D-50 patch SysEx dump",
    extensions: &["syx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
