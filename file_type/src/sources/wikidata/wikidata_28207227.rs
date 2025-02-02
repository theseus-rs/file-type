use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207227: FileFormat = FileFormat {
    id: 28_207_227,
    source_type: SourceType::Wikidata,
    name: "RIFF DIB",
    extensions: &["rdi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
